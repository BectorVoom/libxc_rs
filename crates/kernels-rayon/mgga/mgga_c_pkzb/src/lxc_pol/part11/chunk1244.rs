//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1244/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1244(t17351: f64, t17405: f64, t17505: f64, t20705: f64, t25633: f64, t25636: f64, t25734: f64, t25740: f64, t25747: f64, t25750: f64, t25767: f64, t30284: f64, t30287: f64, t30289: f64, t30291: f64, t30294: f64, t30296: f64, t30309: f64, t30311: f64) -> f64 {
    let t30571 = -0.92617777777777777776e0_f64 * t17405 - 0.48204333333333333333e1_f64 * t20705 + 0.3529725e1_f64 * t30289 + 0.6311625e0_f64 * t30291 + 0.794188125e1_f64 * t30294 - 0.473371875e0_f64 * t30296 + t17505 - 0.16068111111111111111e1_f64 * t17351 + 0.20659e1_f64 * t25633 - 0.1549425e1_f64 * t25636 + 0.104195e1_f64 * t25734 - 0.516475e0_f64 * t30284 + 0.1549425e1_f64 * t30287 - 0.125034e1_f64 * t25740 - 0.62517e0_f64 * t25747 - 0.62517e0_f64 * t25750 + 0.104195e1_f64 * t25767 + 0.2366859375e0_f64 * t30309 - 0.473371875e0_f64 * t30311;
    t30571
}
