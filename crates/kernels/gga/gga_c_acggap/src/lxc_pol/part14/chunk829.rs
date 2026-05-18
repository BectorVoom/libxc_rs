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
    let t9656 = t9266 - t9267 - t9269 + F::new(0.18868855373762491241e-2) * t9609 + F::new(0.34299214494455789578e-2) * t9611 - t7463 + t9615 / F::new(32.0) + t9619 / F::new(192.0) - t9623 / F::new(128.0) - t9627 / F::new(384.0) - F::new(0.38203125e-2) * t9631 - F::new(0.21437009059034868486e-3) * t9634 - F::new(0.10718504529517434243e-3) * t9638 + F::new(0.15724046144802076034e-3) * t9642 - F::new(0.31448092289604152068e-3) * t9646 + F::new(0.21437009059034868486e-3) * t9650 - F::new(0.47172138434406228102e-2) * t9654 + t9277 + t9278 + t7516;
    t9656
}
