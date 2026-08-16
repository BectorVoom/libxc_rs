//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 701/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk701(t10003: f64, t2379: f64, t4985: f64, t1707: f64, t665: f64, t903: f64, t2024: f64, t6522: f64, t739: f64, t236: f64, t6108: f64, t1971: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10004 = 0.59871208509319042821e-1_f64 * t10003;
    let t10005 = t4985 * t2379;
    let t10006 = 0.11974241701863808564e0_f64 * t10005;
    let t10007 = t665 * t1707;
    let t10008 = t903 * t10007;
    let t10009 = 0.35922725105591425692e0_f64 * t10008;
    let t10010 = t2024 * t6522;
    let t10011 = t739 * t10010;
    let t10012 = 0.23948483403727617128e0_f64 * t10011;
    let t10013 = t236 * t6108;
    let t10014 = t1971 * t10013;
    (t10004, t10006, t10007, t10009, t10010, t10012, t10014)
}
