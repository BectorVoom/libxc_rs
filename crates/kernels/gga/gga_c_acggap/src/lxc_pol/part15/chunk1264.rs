//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1264/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1264<F: Float>(t36289: F, t36294: F, t37934: F, t37937: F, t37938: F, t37941: F, t37944: F, t37945: F, t40465: F, t40467: F, t40469: F, t40472: F, t40474: F, t40477: F, t40481: F, t40485: F, t40487: F) -> F {
    let t42132 = -F::cast_from(0.25158473831683321656e-2_f64) * t40465 + F::cast_from(0.34299214494455789578e-2_f64) * t40467 + F::cast_from(0.34299214494455789578e-2_f64) * t40469 + t37934 + t37937 - t37938 - F::cast_from(0.75475421495049964965e-2_f64) * t36289 + t37941 - F::cast_from(0.55907719625962937011e-2_f64) * t36294 + t37944 + t37945 + F::cast_from(0.34299214494455789578e-2_f64) * t40472 + F::cast_from(0.85748036236139473944e-3_f64) * t40474 + t40477 / F::new(8.0) + F::cast_from(0.18868855373762491242e-1_f64) * t40481 - F::cast_from(0.75475421495049964966e-2_f64) * t40485 + F::cast_from(0.42874018118069736972e-2_f64) * t40487;
    t42132
}
