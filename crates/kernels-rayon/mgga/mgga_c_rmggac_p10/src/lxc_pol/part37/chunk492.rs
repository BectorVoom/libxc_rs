//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 492/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk492(t13980: f64, t638: f64, t639: f64, t2127: f64, t640: f64, t3080: f64, t321: f64, t262: f64, t7204: f64, t333: f64, t7192: f64, t2060: f64, t2123: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13982 = t638 * t639 * t13980;
    let t13984 = t640 * t2127;
    let t13986 = t638 * t639 * t13984;
    let t13988 = t3080 * t321;
    let t13989 = t262 * t13988;
    let t13990 = t7204 * t13989;
    let t13992 = t3080 * t333;
    let t13993 = t262 * t13992;
    let t13994 = t7192 * t13993;
    let t13996 = t2060 * t2123;
    (t13982, t13984, t13986, t13988, t13989, t13990, t13992, t13993, t13994, t13996)
}
