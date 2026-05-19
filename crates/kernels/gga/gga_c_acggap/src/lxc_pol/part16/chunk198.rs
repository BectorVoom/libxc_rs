//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 198/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk198<F: Float>(t598: F, t606: F, t572: F, t581: F, t585: F, t590: F, t602: F) -> F {
    let t607 = t598 * t606;
    let t609 = t572 / F::new(96.0) + t581 / F::new(384.0) - F::new(0.38203125e-2) * t585 + F::cast_from(0.42874018118069736972e-3_f64) * t590 + F::cast_from(0.10718504529517434243e-3_f64) * t602 - F::cast_from(0.15724046144802076034e-3_f64) * t607;
    t609
}
