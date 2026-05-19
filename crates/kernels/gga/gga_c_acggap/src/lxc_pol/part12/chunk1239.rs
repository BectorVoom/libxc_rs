//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1239/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1239<F: Float>(t33743: F, t33744: F, t638: F, t30029: F, t9171: F, t33175: F, t7942: F, t8406: F, t2176: F, t5368: F, t1620: F, t8331: F) -> (F, F, F, F, F) {
    let t38343 = F::cast_from(0.10408353825846239354e2_f64) * t33743 * t638 * t33744;
    let t38345 = F::cast_from(0.17347256376410398924e1_f64) * t30029 * t9171;
    let t38348 = F::cast_from(0.17347256376410398924e1_f64) * t7942 * t33175 * t8406;
    let t38361 = t2176 * t5368;
    let t38370 = F::cast_from(0.26341796731742046394e1_f64) * t8331 * t1620;
    (t38343, t38345, t38348, t38361, t38370)
}
