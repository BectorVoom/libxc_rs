//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 871/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk871(t1970: f64, t1971: f64, t333: f64, t511: f64, t6182: f64, t352: f64, t515: f64, t236: f64, t6144: f64, t118: f64, t1986: f64, t209: f64, t44586: f64) -> (f64, f64, f64, f64) {
    let t44642 = t1970 * t1971 * t511 * t6182 * t333;
    let t44647 = t1970 * t1971 * t515 * t6182 * t352;
    let t44651 = t1970 * t1971 * t236 * t6144;
    let t44655 = t1986 * t118 * t44586 * t209;
    (t44642, t44647, t44651, t44655)
}
