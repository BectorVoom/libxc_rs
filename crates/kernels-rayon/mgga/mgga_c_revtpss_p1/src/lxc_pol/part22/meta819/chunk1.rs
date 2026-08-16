//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2932/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2932(t14066: f64, t213: f64, t14109: f64, t47603: f64, t9681: f64, t14268: f64, t3915: f64, t686: f64, t72: f64, t14293: f64, t9664: f64, t1444: f64, t2782: f64, t4075: f64, t556: f64, t5774: f64) -> (f64, f64, f64, f64, f64) {
    let t47909 = t213 * t14066;
    let t47913 = t47603 * t14109 * t9681;
    let t47918 = t3915 * t14268 * t72 * t686;
    let t47920 = t14293 * t9664;
    let t47926 = t2782 * t556 * t4075 * t5774 * t1444;
    (t47909, t47913, t47918, t47920, t47926)
}
