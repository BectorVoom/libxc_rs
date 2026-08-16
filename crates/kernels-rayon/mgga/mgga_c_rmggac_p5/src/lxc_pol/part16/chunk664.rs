//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 664/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk664(t570: f64, t698: f64, t352: f64, t321: f64, t9540: f64, t333: f64, t338: f64, t9486: f64, t118: f64, t326: f64, t4669: f64, t5148: f64, t5155: f64, t5266: f64, t7826: f64, t7832: f64, t7842: f64, t8242: f64, t8243: f64, t8940: f64, t8944: f64, t8966: f64, t9302: f64) -> (f64, f64, f64) {
    let t9551 = t698 * t570;
    let t9552 = t9551 * t352;
    let t9555 = t9540 * t321;
    let t9558 = t9540 * t333;
    let t9565 = t338 * t9486;
    let t9566 = t118 * t9565;
    let t9568 = t9551 * t321;
    let t9571 = t9551 * t333;
    let t9574 = t8242 - t8243 - 0.2993560425465952141e-1_f64 * t8944 - 0.59871208509319042821e-1_f64 * t326 * t9302 + 0.11974241701863808564e0_f64 * t8940 * t9552 - 0.17961362552795712846e0_f64 * t4669 * t9555 + 0.23948483403727617128e0_f64 * t5155 * t9558 - 0.8980681276397856423e-1_f64 * t8966 + 0.54549323308490683461e-1_f64 * t7826 - 0.72732431077987577948e-1_f64 * t7832 - 0.18183107769496894487e-1_f64 * t7842 + 0.19957069503106347607e-1_f64 * t9566 - 0.11974241701863808564e0_f64 * t5148 * t9568 + 0.11974241701863808564e0_f64 * t5266 * t9571;
    (t9551, t9565, t9574)
}
