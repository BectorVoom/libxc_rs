//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 711/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk711(t14059: f64, t14371: f64, t69452: f64, t739: f64, t1986: f64, t2088: f64, t13806: f64, t7508: f64, t3154: f64, t7939: f64, t13809: f64, t7335: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69870 = t14371 * t14059;
    let t69871 = 0.1226351426503095703e-4_f64 * t69870;
    let t69894 = t739 * t69452;
    let t69904 = t1986 * t2088;
    let t69907 = t7508 * t13806;
    let t69924 = t7939 * t3154;
    let t69934 = t7335 * t13809;
    (t69871, t69894, t69904, t69907, t69924, t69934)
}
