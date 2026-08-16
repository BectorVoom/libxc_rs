//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 989/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk989(t2350: f64, t794: f64, t262: f64, t35810: f64, t321: f64, t8712: f64, t7785: f64, t839: f64, t35879: f64, t8708: f64, t7844: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40893 = t2350 * t794;
    let t40894 = t262 * t40893;
    let t40895 = t35810 * t40894;
    let t40897 = t8712 * t321;
    let t40898 = t262 * t40897;
    let t40899 = t7785 * t40898;
    let t40901 = t2350 * t839;
    let t40902 = t262 * t40901;
    let t40903 = t35879 * t40902;
    let t40905 = t8708 * t321;
    let t40906 = t262 * t40905;
    let t40907 = t7844 * t40906;
    (t40893, t40894, t40895, t40897, t40898, t40899, t40901, t40902, t40903, t40905, t40906, t40907)
}
