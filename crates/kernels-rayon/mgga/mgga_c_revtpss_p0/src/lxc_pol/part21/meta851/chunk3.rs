//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 3200/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk3200(t17794: f64, t372: f64, t3584: f64, t606: f64, t1261: f64, t17203: f64, t3172: f64, t43766: f64, t44361: f64, t12916: f64, t17419: f64, t5340: f64) -> (f64, f64, f64, f64, f64) {
    let t58960 = t372 * t17794;
    let t58969 = t3584 * t606;
    let t58975 = t1261 * t3172 * t17203;
    let t58983 = t44361 * t43766;
    let t58997 = t5340 * t12916 * t17419;
    (t58960, t58969, t58975, t58983, t58997)
}
