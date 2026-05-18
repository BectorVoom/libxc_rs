//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1075/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1075<F: Float>(t872: F, t9380: F, t2132: F, t2138: F, t2385: F, t879: F, t33743: F, t33744: F, t638: F, t30029: F, t9171: F, t33175: F, t7942: F, t8406: F) -> (F, F, F, F, F) {
    let t38324 = F::new(0.13170898365871023197e1) * t9380 * t872;
    let t38329 = t2138 * t2132 * t2385 * t879;
    let t38343 = F::new(0.10408353825846239354e2) * t33743 * t638 * t33744;
    let t38345 = F::new(0.17347256376410398924e1) * t30029 * t9171;
    let t38348 = F::new(0.17347256376410398924e1) * t7942 * t33175 * t8406;
    (t38324, t38329, t38343, t38345, t38348)
}
