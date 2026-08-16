//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 666/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk666(t1587: f64, t26: f64, t1652: f64, t880: f64, t892: f64, t1679: f64, t2144: f64, t14100: f64, t14208: f64, t3116: f64, t4443: f64, t14045: f64, t14123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t55986 = t26 * t1587;
    let t56399 = t26 * t1652;
    let t56828 = t892 * t880;
    let t56963 = t1679 * t2144;
    let t61965 = t1679 * t880;
    let t68336 = 0.39726959900411316772e-4_f64 * t14100;
    let t68354 = 0.15965655602485078085e0_f64 * t14208;
    let t68355 = t4443 * t3116;
    let t68357 = t14045 * t68355 * t14123;
    (t55986, t56399, t56828, t56963, t61965, t68336, t68354, t68355, t68357)
}
