//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 625/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk625<F: Float>(t3022: F, t983: F, t2986: F, t2988: F, t973: F, t981: F, t3006: F, t964: F, t3011: F, t3014: F, t2846: F, t2848: F, t2855: F, t2860: F, t2864: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3024 = F::cast_from(0.11696447245269292414e1_f64) * t3022 * t983;
    let t3026 = t2986 * t2988 * t973;
    let t3028 = F::cast_from(0.11696447245269292414e1_f64) * t981 * t3026;
    let t3030 = t964 * t3006 * t973;
    let t3032 = F::cast_from(0.5848223622634646207e0_f64) * t981 * t3030;
    let t3033 = t3011 * t2988;
    let t3034 = t3033 * t3014;
    let t3036 = F::cast_from(0.17315859105681463759e2_f64) * t981 * t3034;
    let t3037 = F::cast_from(0.11111111111111111111e-1_f64) * t2846;
    let t3042 = t3037 + F::cast_from(0.55555555555555555556e-2_f64) * t2848 - F::cast_from(0.55555555555555555555e-2_f64) * t2855 + F::cast_from(0.16666666666666666667e-1_f64) * t2860 - F::cast_from(0.83333333333333333333e-2_f64) * t2864;
    (t3024, t3026, t3028, t3030, t3032, t3034, t3036, t3037, t3042)
}
