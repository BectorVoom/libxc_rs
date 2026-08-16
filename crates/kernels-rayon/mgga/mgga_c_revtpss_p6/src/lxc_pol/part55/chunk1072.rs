//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1072/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1072(t7574: f64, t8435: f64, t2247: f64, t196: f64, t197: f64, t7687: f64, t4147: f64, t7535: f64, t2056: f64, t27060: f64, t29432: f64, t32386: f64, t32388: f64, t32393: f64, t32395: f64, t32396: f64, t32397: f64, t32398: f64, t32402: f64, t32404: f64, t7359: f64, t7367: f64, t7586: f64, t7591: f64) -> (f64, f64, f64, f64, f64) {
    let t32805 = t8435 * t7574;
    let t32806 = t2247 * t32805;
    let t32822 = t7687 * t196 * t197;
    let t33183 = t4147 * t7535;
    let t33245 = -t2056 * t27060 - t2056 * t29432 - t7359 * t7591 - t7367 * t7586 - t32386 - t32388 - t32393 - t32395 - t32396 - t32397 - t32398 - t32402 - t32404;
    (t32805, t32806, t32822, t33183, t33245)
}
