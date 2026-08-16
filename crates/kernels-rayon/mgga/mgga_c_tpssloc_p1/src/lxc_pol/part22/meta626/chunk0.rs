//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2160/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2160(t16060: f64, t3865: f64, t1831: f64, t40292: f64, t12345: f64, t5314: f64, t40018: f64, t5223: f64, t12282: f64, t5234: f64, t12189: f64, t5227: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t53906 = t16060 * t3865;
    let t53917 = t40292 * t1831;
    let t53918 = 119.0_f64 / 1152.0_f64 * t53917;
    let t53919 = t12345 * t5314;
    let t53920 = 119.0_f64 / 1152.0_f64 * t53919;
    let t53927 = t40018 * t5223;
    let t53928 = 35.0_f64 / 24.0_f64 * t53927;
    let t53945 = t5234 * t12282;
    let t53984 = t12189 * t5227;
    (t53906, t53918, t53920, t53928, t53945, t53984)
}
