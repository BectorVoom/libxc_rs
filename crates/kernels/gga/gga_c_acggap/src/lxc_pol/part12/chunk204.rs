//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 204/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk204<F: Float>(t572: F, t581: F, t585: F, t590: F, t602: F, t607: F) -> F {
    let t633 = t572 / F::cast_from(48.0_f64) + t581 / F::cast_from(192.0_f64) - F::cast_from(0.7640625e-2_f64) * t585 + F::cast_from(0.85748036236139473944e-3_f64) * t590 + F::cast_from(0.21437009059034868486e-3_f64) * t602 - F::cast_from(0.31448092289604152069e-3_f64) * t607;
    t633
}
