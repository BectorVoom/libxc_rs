//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1369/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1369(t5883: f64, t8151: f64, t114434: f64, t114436: f64, t114438: f64, t114440: f64, t114442: f64, t114445: f64, t114451: f64, t114455: f64, t114746: f64, t114753: f64, t114755: f64, t114757: f64, t114759: f64, t114765: f64, t114768: f64, t2163: f64, t22578: f64, t22634: f64, t22639: f64, t508: f64, t5884: f64, t7586: f64, t8233: f64) -> (f64, f64) {
    let t116732 = t8151 * t5883;
    let t116735 = -6.0_f64 * t116732 * t508 - 6.0_f64 * t2163 * t22639 - 6.0_f64 * t22578 * t7586 - 2.0_f64 * t22634 * t7586 - 6.0_f64 * t5884 * t8233 - t114434 - t114436 - t114438 - t114440 - t114442 + t114445 + t114451 - t114455 + t114746 + t114753 + t114755 + t114757 - t114759 - t114765 + t114768;
    (t116732, t116735)
}
