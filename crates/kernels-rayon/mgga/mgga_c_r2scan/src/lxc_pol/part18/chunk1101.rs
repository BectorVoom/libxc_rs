//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1101/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1101(t19839: f64, t20: f64, t3293: f64, t10810: f64, t1592: f64, t8160: f64, t2196: f64, t7615: f64, t2147: f64, t2608: f64, t38168: f64, t10855: f64, t128: f64, t512: f64) -> (f64, f64, f64, f64, f64) {
    let t39849 = t3293 * t19839 * t20;
    let t39854 = t1592 * t10810 * t8160;
    let t39855 = 0.69345773920434148506e0_f64 * t39854;
    let t39857 = t2196 * t10810 * t7615;
    let t39858 = 0.27738309568173659402e1_f64 * t39857;
    let t39882 = t2147 * t38168 * t2608;
    let t39885 = t512 * t10855 * t128;
    (t39849, t39855, t39858, t39882, t39885)
}
