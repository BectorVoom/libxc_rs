//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1273/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1273(t32633: f64, t7898: f64, t121593: f64, t2014: f64, t7900: f64, t28189: f64, t8698: f64, t32630: f64, t125428: f64, t2107: f64, t125950: f64, t1843: f64, t2052: f64, t27830: f64, t32107: f64, t32109: f64, t32112: f64, t32609: f64, t5517: f64, t7357: f64, t7883: f64, t8463: f64, t8627: f64) -> f64 {
    let t128898 = t7898 * t32633;
    let t128903 = 3.0_f64 * t2014 * t121593 * t7900;
    let t128904 = t8698 * t28189;
    let t128906 = 3.0_f64 * t7898 * t32630;
    let t128910 = t2014 * t2107 * t125428;
    let t128911 = -t1843 * t32609 - t2052 * t27830 - t5517 * t8627 - t7357 * t7883 - t125950 - t128898 + t128903 - t128904 + t128906 - t128910 - t32107 - t32109 - t32112 - t8463;
    t128911
}
