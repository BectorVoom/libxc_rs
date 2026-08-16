//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 493/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk493(t2009: f64, t301: f64, t761: f64, t758: f64, t132: f64, t747: f64, t288: f64, t749: f64) -> (f64, f64, f64, f64, f64) {
    let t2011 = t301 * t2009 * t761;
    let t2012 = t758 * t2011;
    let t2016 = 1.0_f64 / t747 / t132;
    let t2018 = 1.0_f64 / t749 / t288;
    let t2019 = t2016 * t2018;
    (t2011, t2012, t2016, t2018, t2019)
}
