//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 210/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk210<F: Float>(t265: F, t272: F, t680: F, t286: F, t264: F) -> (F, F, F, F) {
    let t682 = t265 * t680 * t272;
    let t683 = t286 * t682;
    let t684 = F::cast_from(0.5848223622634646207e0_f64) * t683;
    let t685 = t264 * t264;
    let t686 = F::new(1.0) / t685;
    (t682, t684, t685, t686)
}
