//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 435/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk435(t202: f64, t4443: f64, t1228: f64, t1237: f64, t3046: f64, t31: f64, t212: f64, t222: f64, t1224: f64, t28: f64, t492: f64, t1233: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4444 = t202 * t4443;
    let t4451 = t1228 * t1237;
    let t4457 = t31 * t3046;
    let t4460 = 0.92481467875469997376e0_f64 * t212 * t4457 * t222;
    let t4461 = t1224 * t28;
    let t4462 = t212 * t4461;
    let t4463 = t4462 * t492;
    let t4465 = t1228 * t1233;
    (t4444, t4451, t4460, t4461, t4462, t4463, t4465)
}
