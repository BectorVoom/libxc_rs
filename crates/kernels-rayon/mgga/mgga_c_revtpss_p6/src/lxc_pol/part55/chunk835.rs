//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 835/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk835(t4147: f64, t5778: f64, t1882: f64, t4003: f64, t136: f64, t550: f64, t220: f64, t221: f64, t5627: f64, t1398: f64, t125: f64, t5591: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t13648 = t5778 * t4147;
    let t13790 = t1882 * t4003;
    let t13846 = t550 * t136;
    let t13847 = t13846 * t220;
    let t13877 = t221 * t5627;
    let t13926 = t1882 * t1398;
    let t13975 = t125 * t5591;
    (t13648, t13790, t13846, t13847, t13877, t13926, t13975)
}
