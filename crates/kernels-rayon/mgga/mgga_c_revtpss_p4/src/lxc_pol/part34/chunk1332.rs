//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1332/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1332(t114752: f64, t2035: f64, t29499: f64, t7898: f64, t29495: f64, t29506: f64, t7937: f64, t2014: f64, t2034: f64, t86791: f64, t114363: f64, t114434: f64, t114436: f64, t114438: f64, t114440: f64, t114442: f64, t114445: f64, t114451: f64, t114455: f64, t114746: f64, t1843: f64, t2011: f64, t22578: f64, t23094: f64, t29573: f64, t508: f64, t6765: f64, t6934: f64, t6985: f64, t7725: f64, t7894: f64) -> f64 {
    let t114753 = t114752 * t2035;
    let t114755 = 18.0_f64 * t7898 * t29499;
    let t114757 = 9.0_f64 * t7898 * t29495;
    let t114759 = 3.0_f64 * t29506 * t7937;
    let t114765 = 6.0_f64 * t2014 * t2034 * t86791;
    let t114766 = -6.0_f64 * t114363 * t508 - 6.0_f64 * t1843 * t29573 + t2011 * t23094 - 6.0_f64 * t22578 * t6985 - 3.0_f64 * t6765 * t7725 + 3.0_f64 * t6934 * t7894 - t114434 - t114436 - t114438 - t114440 - t114442 + t114445 + t114451 - t114455 + t114746 + t114753 + t114755 + t114757 - t114759 - t114765;
    t114766
}
