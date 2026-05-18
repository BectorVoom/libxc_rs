//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 538/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk538<F: Float>(t300: F, t3018: F, t2980: F, t960: F, t983: F, t2986: F, t2988: F, t973: F, t981: F, t3006: F, t964: F, t3011: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3019 = t300 * t3018;
    let t3021 = F::new(0.19751673498613801407e-1) * t300 * t2980;
    let t3022 = t300 * t960;
    let t3024 = F::new(0.11696447245269292414e1) * t3022 * t983;
    let t3026 = t2986 * t2988 * t973;
    let t3028 = F::new(0.11696447245269292414e1) * t981 * t3026;
    let t3030 = t964 * t3006 * t973;
    let t3032 = F::new(0.5848223622634646207e0) * t981 * t3030;
    let t3033 = t3011 * t2988;
    (t3019, t3021, t3022, t3024, t3026, t3028, t3030, t3032, t3033)
}
