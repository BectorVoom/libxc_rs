//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 876/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk876(t1184: f64, t12747: f64, t1190: f64, t3378: f64, t3430: f64, t1177: f64, t12727: f64, t1017: f64, t1459: f64, t384: f64, t398: f64, t879: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12748 = t12747 * t1184;
    let t12750 = t12747 * t1190;
    let t12752 = t3378 * t3430;
    let t12753 = t12752 * t1177;
    let t12755 = t12727 * t1184;
    let t12762 = t384 * t398 * t1459 * t1017 * t879;
    (t12748, t12750, t12752, t12753, t12755, t12762)
}
