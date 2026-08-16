//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2747/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2747(t3766: f64, t6564: f64, t17191: f64, t5219: f64, t21342: f64, t473: f64, t1770: f64, t17845: f64, t17852: f64, t17948: f64, t13147: f64, t1811: f64, t460: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t72370 = t6564 * t3766;
    let t72386 = t5219 * t17191;
    let t72397 = t473 * t21342;
    let t72429 = t1770 * t17845;
    let t72432 = t1770 * t17852;
    let t72435 = t1770 * t17948;
    let t72686 = t460 * t13147 * t1811;
    (t72370, t72386, t72397, t72429, t72432, t72435, t72686)
}
