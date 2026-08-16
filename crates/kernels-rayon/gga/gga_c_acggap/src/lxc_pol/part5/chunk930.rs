//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 930/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk930(t1195: f64, t3670: f64, t1036: f64, t1037: f64, t3266: f64, t386: f64, t1098: f64, t3237: f64, t1092: f64, t1086: f64, t941: f64, t980: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14309 = t3670 * t1195;
    let t14313 = t1036 * t386 * t3266 * t1037;
    let t14339 = t3237 * t1098;
    let t14341 = t3237 * t1092;
    let t14343 = t3237 * t1086;
    let t14345 = t980 * t941;
    (t14309, t14313, t14339, t14341, t14343, t14345)
}
