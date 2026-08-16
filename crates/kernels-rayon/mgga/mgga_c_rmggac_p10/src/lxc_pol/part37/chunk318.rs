//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 318/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk318(t262: f64, t3068: f64, t2500: f64, t2060: f64, t664: f64, t305: f64, t128: f64, t838: f64, t28: f64, t3046: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3069 = t3068 * t262;
    let t3070 = t2500 * t3069;
    let t3072 = t2060 * t664;
    let t3074 = 0.2993560425465952141e-1_f64 * t305 * t3072;
    let t3075 = t838 * t128;
    let t3076 = t28 * t3046;
    (t3069, t3070, t3072, t3074, t3075, t3076)
}
