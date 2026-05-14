//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 769/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk769<F: Float>(t8529: F, t9333: F, t8543: F, t8546: F, t8549: F, t8552: F, t9341: F, t9344: F, t8563: F, t8565: F, t8569: F, t9353: F, t8572: F, t8574: F, t8578: F, t8583: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42429 = 0.5454932330849068346e-1 * t8529;
    let t42434 = 0.11974241701863808564e0 * t9333;
    let t42435 = 0.11974241701863808564e0 * t8543;
    let t42436 = 0.35922725105591425692e0 * t8546;
    let t42437 = 0.71845450211182851384e0 * t8549;
    let t42438 = 0.17961362552795712846e0 * t8552;
    let t42444 = 0.79828278012425390428e-1 * t9341;
    let t42445 = 0.4726e1 * t9344;
    let t42450 = 0.5454932330849068346e-1 * t8563;
    let t42451 = 0.13637330827122670865e-1 * t8565;
    let t42452 = 0.13637330827122670865e-1 * t8569;
    let t42454 = 0.11974241701863808564e0 * t9353;
    let t42455 = 0.1702583995731913576e-4 * t8572;
    let t42456 = 0.5107751987195740728e-4 * t8574;
    let t42459 = 0.1702583995731913576e-4 * t8578;
    let t42460 = 0.1702583995731913576e-4 * t8583;
    (t42429, t42434, t42435, t42436, t42437, t42438, t42444, t42445, t42450, t42451, t42452, t42454, t42455, t42456, t42459, t42460)
}
