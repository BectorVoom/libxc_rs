//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 818/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk818(t1212: f64, t1970: f64, t1971: f64, t209: f64, t515: f64, t570: f64, t7244: f64, t8447: f64, t321: f64, t14243: f64, t16503: f64, t333: f64, t8440: f64) -> (f64, f64, f64, f64) {
    let t38412 = t1970 * t1971 * t515 * t570 * t1212 * t209;
    let t38414 = t7244 * t8447;
    let t38415 = 0.19863479950205658386e-4_f64 * t38414;
    let t38416 = t209 * t321;
    let t38420 = t16503 * t14243 * t8440 * t38416 * t333;
    (t38412, t38415, t38416, t38420)
}
