//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 661/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk661(t515: f64, t9163: f64, t1971: f64, t1970: f64, t209: f64, t476: f64, t618: f64, t236: f64, t7231: f64, t739: f64, t8994: f64, t8988: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t9164 = t515 * t9163;
    let t9165 = t1971 * t9164;
    let t9166 = t1970 * t9165;
    let t9169 = t618 * t476 * t209;
    let t9170 = t236 * t9169;
    let t9171 = t7231 * t9170;
    let t9172 = t1970 * t9171;
    let t9174 = t739 * t8994;
    let t9176 = t739 * t8988;
    (t9165, t9166, t9171, t9172, t9174, t9176)
}
