//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 838/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk838(t39700: f64, t797: f64, t40897: f64, t5271: f64, t40920: f64, t5162: f64, t38568: f64, t4669: f64, t1587: f64, t2064: f64, t793: f64, t118: f64, t2001: f64, t352: f64, t38523: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41523 = t797 * t39700;
    let t41531 = t5271 * t40897;
    let t41534 = t5162 * t40920;
    let t41536 = t4669 * t38568;
    let t41548 = t2064 * t1587;
    let t41549 = t793 * t41548;
    let t41576 = t2001 * t118 * t38523 * t352;
    (t41523, t41531, t41534, t41536, t41548, t41549, t41576)
}
