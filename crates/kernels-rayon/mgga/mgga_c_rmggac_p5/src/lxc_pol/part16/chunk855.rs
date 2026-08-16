//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 855/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk855(t8563: f64, t8565: f64, t8569: f64, t9353: f64, t8572: f64, t8574: f64, t8578: f64, t8583: f64, t8585: f64, t8588: f64, t8590: f64, t8593: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42450 = 0.5454932330849068346e-1_f64 * t8563;
    let t42451 = 0.13637330827122670865e-1_f64 * t8565;
    let t42452 = 0.13637330827122670865e-1_f64 * t8569;
    let t42454 = 0.11974241701863808564e0_f64 * t9353;
    let t42455 = 0.1702583995731913576e-4_f64 * t8572;
    let t42456 = 0.5107751987195740728e-4_f64 * t8574;
    let t42459 = 0.1702583995731913576e-4_f64 * t8578;
    let t42460 = 0.1702583995731913576e-4_f64 * t8583;
    let t42461 = 0.5107751987195740728e-4_f64 * t8585;
    let t42462 = 0.5107751987195740728e-4_f64 * t8588;
    let t42463 = 0.5107751987195740728e-4_f64 * t8590;
    let t42464 = 0.5107751987195740728e-4_f64 * t8593;
    (t42450, t42451, t42452, t42454, t42455, t42456, t42459, t42460, t42461, t42462, t42463, t42464)
}
