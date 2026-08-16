//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 737/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk737(t34884: f64, t7239: f64, t16156: f64, t7746: f64, t1990: f64, t34881: f64, t7234: f64, t2185: f64, t7690: f64, t1997: f64, t7414: f64, t7696: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t34885 = t34884 * t7239;
    let t34887 = t16156 * t7746;
    let t34889 = t34881 * t1990;
    let t34894 = t34884 * t7234;
    let t34902 = t7690 * t2185;
    let t34903 = t34902 * t1997;
    let t34905 = t7414 * t7696;
    (t34885, t34887, t34889, t34894, t34902, t34903, t34905)
}
