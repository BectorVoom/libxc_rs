//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1242/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1242<F: Float>(t31160: F, t35373: F, t37519: F, t37522: F, t37523: F, t37524: F, t37525: F, t37526: F, t39907: F, t39910: F, t39914: F, t39919: F, t39923: F, t39925: F, t39928: F, t39930: F, t39932: F, t39934: F) -> F {
    let t41856 = F::new(0.4584375e-1) * t39907 + F::new(0.305625e-1) * t39910 - F::cast_from(0.34299214494455789578e-2_f64) * t31160 - F::cast_from(0.85748036236139473944e-3_f64) * t39914 - t35373 + F::cast_from(0.64311027177104605458e-2_f64) * t39919 - F::cast_from(0.6431102717710460546e-2_f64) * t39923 - t37519 - F::new(11.0) / F::new(288.0) * t39925 + t37522 + t37523 + t37524 - t37525 + F::new(0.4584375e-1) * t39928 + F::cast_from(0.13719685797782315831e-1_f64) * t39930 - t37526 - F::cast_from(0.68598428988911579156e-2_f64) * t39932 - F::cast_from(0.68598428988911579156e-2_f64) * t39934;
    t41856
}
