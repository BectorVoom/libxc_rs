//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1016/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1016<F: Float>(t7637: F, t8491: F, t1967: F, t8536: F, t4708: F, t7561: F, t4439: F, t7822: F, t4681: F, t4443: F, t30543: F, t8661: F) -> (F, F, F, F, F, F, F) {
    let t34011 = t7637 * t8491;
    let t34013 = t1967 * t8536;
    let t34014 = F::new(0.64311027177104605458e-2) * t34013;
    let t34015 = t7561 * t4708;
    let t34017 = t7822 * t4439;
    let t34019 = t7822 * t4681;
    let t34021 = t7822 * t4443;
    let t34023 = t30543 * t8661;
    (t34011, t34014, t34015, t34017, t34019, t34021, t34023)
}
