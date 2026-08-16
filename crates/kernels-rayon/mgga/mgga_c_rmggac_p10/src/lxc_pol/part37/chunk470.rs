//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 470/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk470(t262: f64, t321: f64, t3068: f64, t7282: f64, t333: f64, t12200: f64, t2084: f64, t664: f64, t27: f64, t2145: f64, t2020: f64, t3061: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t13797 = t262 * t321;
    let t13798 = t3068 * t13797;
    let t13799 = t7282 * t13798;
    let t13801 = t262 * t333;
    let t13802 = t3068 * t13801;
    let t13803 = t12200 * t13802;
    let t13805 = t2084 * t664;
    let t13806 = t27 * t13805;
    let t13807 = t2145 * t13806;
    let t13809 = t2020 * t3061;
    (t13797, t13798, t13799, t13801, t13802, t13803, t13806, t13807, t13809)
}
