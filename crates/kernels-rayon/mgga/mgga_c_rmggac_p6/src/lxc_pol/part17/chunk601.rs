//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 601/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk601(t270: f64, t535: f64, t2039: f64, t638: f64, t31: f64, t2046: f64, t2050: f64, t2305: f64, t7255: f64, t236: f64, t5605: f64, t1971: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8486 = t535 * t270;
    let t8488 = t638 * t2039 * t8486;
    let t8490 = t535 * t31;
    let t8492 = t2046 * t2050 * t8490;
    let t8494 = t7255 * t2305;
    let t8496 = t236 * t5605;
    let t8497 = t1971 * t8496;
    (t8486, t8488, t8490, t8492, t8494, t8497)
}
