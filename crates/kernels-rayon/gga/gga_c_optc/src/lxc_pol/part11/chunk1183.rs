//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1183/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1183(t15015: f64, t4535: f64, t15011: f64, t4229: f64, t1219: f64, t176: f64, t17612: f64, t3116: f64, t46945: f64, t5324: f64, t17941: f64, t3103: f64, t45304: f64) -> (f64, f64, f64, f64, f64) {
    let t53911 = t4535 * t15015;
    let t53914 = t15011 * t4229;
    let t53918 = t176 * t17612 * t1219;
    let t53950 = t3116 * t46945 * t5324;
    let t53953 = t3103 * t45304 * t17941;
    (t53911, t53914, t53918, t53950, t53953)
}
