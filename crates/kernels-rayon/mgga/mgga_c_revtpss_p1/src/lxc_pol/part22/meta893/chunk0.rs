//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3081/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3081(t1058: f64, t15859: f64, t3201: f64, t4794: f64, t15866: f64, t15888: f64, t4798: f64, t343: f64, t44: f64, t816: f64, t11821: f64, t65: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t53298 = t15859 * t1058;
    let t53300 = t4794 * t3201;
    let t53302 = t15866 * t1058;
    let t53308 = t15888 * t1058;
    let t53317 = t4798 * t3201;
    let t53320 = t44 * t343 * t816;
    let t53321 = t65 * t11821;
    (t53298, t53300, t53302, t53308, t53317, t53320, t53321)
}
