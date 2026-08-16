//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 851/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk851(t236: f64, t9971: f64, t240: f64, t812: f64, t232: f64, t2632: f64, t9660: f64, t819: f64, t820: f64, t2639: f64, t2686: f64, t2697: f64, t2703: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9972 = t9971 * t236;
    let t9973 = t9972 * t240;
    let t9974 = t812 * t9973;
    let t9975 = t2632 * t232;
    let t9976 = t9660 * t9975;
    let t9978 = t819 * t820 * t9976;
    let t9981 = t9660 * t2632;
    let t9983 = t819 * t820 * t9981;
    let t9986 = t2639 * t2686;
    let t9988 = t2697 * t2703;
    (t9972, t9974, t9975, t9976, t9978, t9981, t9983, t9986, t9988)
}
