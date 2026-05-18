//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 855/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk855<F: Float>(t8563: F, t8565: F, t8569: F, t9353: F, t8572: F, t8574: F, t8578: F, t8583: F, t8585: F, t8588: F, t8590: F, t8593: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t42450 = F::new(0.5454932330849068346e-1) * t8563;
    let t42451 = F::new(0.13637330827122670865e-1) * t8565;
    let t42452 = F::new(0.13637330827122670865e-1) * t8569;
    let t42454 = F::new(0.11974241701863808564e0) * t9353;
    let t42455 = F::new(0.1702583995731913576e-4) * t8572;
    let t42456 = F::new(0.5107751987195740728e-4) * t8574;
    let t42459 = F::new(0.1702583995731913576e-4) * t8578;
    let t42460 = F::new(0.1702583995731913576e-4) * t8583;
    let t42461 = F::new(0.5107751987195740728e-4) * t8585;
    let t42462 = F::new(0.5107751987195740728e-4) * t8588;
    let t42463 = F::new(0.5107751987195740728e-4) * t8590;
    let t42464 = F::new(0.5107751987195740728e-4) * t8593;
    (t42450, t42451, t42452, t42454, t42455, t42456, t42459, t42460, t42461, t42462, t42463, t42464)
}
