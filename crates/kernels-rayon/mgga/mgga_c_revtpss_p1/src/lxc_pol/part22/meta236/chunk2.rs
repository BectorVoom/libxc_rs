//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1483/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1483(t1868: f64, t4140: f64, t3854: f64, t3859: f64, t3862: f64, t3867: f64, t3871: f64, t3873: f64, t4030: f64, t4035: f64, t4037: f64, t4042: f64, t4139: f64, t5634: f64, t5637: f64, t5639: f64, t5640: f64, t5641: f64) -> f64 {
    let t5783 = t4140 * t1868;
    let t5786 = 3.0_f64 * t4139 * t5783 + t3854 + t3859 - t3862 - t3867 + t3871 + t3873 + t4030 - t4035 - t4037 + t4042 + t5634 - t5637 - t5639 - t5640 - t5641;
    t5786
}
