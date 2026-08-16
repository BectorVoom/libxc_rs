//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 1055/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk1055(t1502: f64, t16503: f64, t34976: f64, t571: f64, t40771: f64, t9147: f64, t10066: f64, t34764: f64, t1685: f64, t2405: f64, t40750: f64, t41571: f64, t41579: f64, t41651: f64, t47295: f64, t47302: f64, t47306: f64, t47310: f64, t47312: f64, t47316: f64, t47321: f64, t47325: f64, t47327: f64, t4965: f64, t530: f64, t72: f64, t9852: f64) -> f64 {
    let t47331 = t16503 * t34976 * t571 * t1502;
    let t47333 = t40771 * t9147;
    let t47335 = t34764 * t10066;
    let t47338 = t40750 - 0.2993560425465952141e-1_f64 * t47295 - 0.4726e1_f64 * t530 * t41651 + 2.0_f64 * t72 * t1685 * t2405 - 0.42564599893297839398e-5_f64 * t47302 + 0.42564599893297839398e-5_f64 * t47306 - 0.38906704589967556326e-4_f64 * t47310 - 0.4726e1_f64 * t47312 - 0.11974241701863808564e0_f64 * t4965 * t9852 + 0.14967802127329760705e-1_f64 * t47316 + 2.0_f64 * t41571 - 0.25538759935978703639e-4_f64 * t47321 - 0.17025839957319135759e-4_f64 * t47325 + 0.85129199786595678796e-5_f64 * t47327 + 0.85129199786595678796e-5_f64 * t47331 + 0.1064114997332445985e-4_f64 * t47333 + 0.1064114997332445985e-4_f64 * t47335 + 0.74488049813271218946e-4_f64 * t41579;
    t47338
}
