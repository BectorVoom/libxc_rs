//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1711/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1711(t1043: f64, t3075: f64, t1045: f64, t3117: f64, t3316: f64, t994: f64, t4891: f64) -> (f64, f64, f64, f64, f64) {
    let t11869 = t3075 * t1043;
    let t11870 = t11869 * t1045;
    let t11871 = t3117 * t11870;
    let t11874 = t994 * t3316;
    let t11875 = t11874 * t4891;
    (t11869, t11870, t11871, t11874, t11875)
}
