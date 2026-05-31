//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1009/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1009<F: Float>(t1980: F, t35500: F, t7476: F, t31262: F, t31277: F, t31279: F, t1988: F, t8486: F, t1967: F, t8838: F, t31285: F, t4360: F, t7741: F) -> (F, F, F, F, F, F, F, F) {
    let t35502 = t1980 * t7476 * t35500;
    let t35506 = F::cast_from(0.26147916666666666666e0_f64) * t31262;
    let t35507 = F::cast_from(0.3973125e0_f64) * t31277;
    let t35508 = F::cast_from(0.264875e0_f64) * t31279;
    let t35513 = t1988 * t8486;
    let t35515 = t1967 * t8838;
    let t35527 = F::cast_from(0.10718504529517434243e-2_f64) * t31285;
    let t35529 = t7741 * t4360;
    (t35502, t35506, t35507, t35508, t35513, t35515, t35527, t35529)
}
