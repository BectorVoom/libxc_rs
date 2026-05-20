//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1550/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1550<F: Float>(t11940: F, t366: F, t1053: F, t3204: F, t1021: F, t3201: F, t1054: F, t2434: F, t371: F, t373: F, t367: F, t1065: F, t675: F) -> (F, F, F, F, F, F, F) {
    let t11941 = t11940 * t366;
    let t11947 = t3204 * t1053;
    let t11956 = t1021 * t3201;
    let t11967 = t1054 * t3201;
    let t11970 = t371 * t2434 * t373;
    let t11972 = F::cast_from(0.63517063878621832551e-4_f64) * t367 * t11970;
    let t11986 = t675 * t1065;
    (t11941, t11947, t11956, t11967, t11970, t11972, t11986)
}
