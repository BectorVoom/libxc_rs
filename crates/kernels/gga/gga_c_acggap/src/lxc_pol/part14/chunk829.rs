//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 829/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk829<F: Float>(t7575: F, t9653: F, t7463: F, t7516: F, t9266: F, t9267: F, t9269: F, t9277: F, t9278: F, t9609: F, t9611: F, t9615: F, t9619: F, t9623: F, t9627: F, t9631: F, t9634: F, t9638: F, t9642: F, t9646: F, t9650: F) -> F {
    let t9654 = t7575 * t9653;
    let t9656 = t9266 - t9267 - t9269 + F::cast_from(0.18868855373762491241e-2_f64) * t9609 + F::cast_from(0.34299214494455789578e-2_f64) * t9611 - t7463 + t9615 / F::cast_from(32.0_f64) + t9619 / F::cast_from(192.0_f64) - t9623 / F::cast_from(128.0_f64) - t9627 / F::cast_from(384.0_f64) - F::cast_from(0.38203125e-2_f64) * t9631 - F::cast_from(0.21437009059034868486e-3_f64) * t9634 - F::cast_from(0.10718504529517434243e-3_f64) * t9638 + F::cast_from(0.15724046144802076034e-3_f64) * t9642 - F::cast_from(0.31448092289604152068e-3_f64) * t9646 + F::cast_from(0.21437009059034868486e-3_f64) * t9650 - F::cast_from(0.47172138434406228102e-2_f64) * t9654 + t9277 + t9278 + t7516;
    t9656
}
