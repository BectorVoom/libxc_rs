//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1227/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1227(t141: f64, t2087: f64, t55893: f64, t1260: f64, t16287: f64, t55933: f64, t659: f64, t21874: f64, t21878: f64, t21884: f64, t21887: f64, t21891: f64, t21895: f64, t21899: f64, t21903: f64, t21907: f64, t55862: f64, t55875: f64, t55878: f64) -> (f64, f64, f64, f64) {
    let t56229 = t2087 * t141 * t55893;
    let t56232 = t1260 * t16287;
    let t56252 = t659 * t141 * t55933;
    let t56255 = t55862 - t21874 - t21878 + t21884 + t21887 + t21891 + t21895 - t21899 - t21903 - t21907 + t55875 + t55878;
    (t56229, t56232, t56252, t56255)
}
