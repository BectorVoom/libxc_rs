//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1088/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1088(t1256: f64, t6595: f64, t6598: f64, t17183: f64, t5330: f64, t1811: f64, t5219: f64, t1284: f64, t6564: f64, t6688: f64, t73: f64, t3766: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t21285 = t6595 * t1256;
    let t21287 = t6598 * t1256;
    let t21306 = t17183 * t5330;
    let t21394 = t5219 * t1811;
    let t21439 = t6564 * t1284;
    let t21442 = t6688 * t73;
    let t21451 = t3766 * t1811;
    (t21285, t21287, t21306, t21394, t21439, t21442, t21451)
}
