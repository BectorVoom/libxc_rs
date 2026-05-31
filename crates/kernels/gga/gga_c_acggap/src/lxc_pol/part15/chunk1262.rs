//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1262/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1262<F: Float>(t32923: F, t36127: F, t36129: F, t36135: F, t36137: F, t36139: F, t36141: F, t37872: F, t37875: F, t37876: F, t37888: F, t37892: F, t40425: F, t40427: F, t40431: F, t40436: F, t40442: F, t40446: F) -> F {
    let t42110 = -t40425 / F::cast_from(96.0_f64) + t37872 + F::cast_from(0.56606566121287473722e-1_f64) * t40427 + F::cast_from(0.31448092289604152069e-2_f64) * t40431 + F::cast_from(0.15095084299009992993e-1_f64) * t36127 - F::cast_from(0.85748036236139473944e-3_f64) * t36129 - t37875 - t37876 - F::cast_from(0.11433071498151929859e-2_f64) * t36135 - F::cast_from(0.12579236915841660828e-2_f64) * t40436 + F::cast_from(0.79249192569802463215e-1_f64) * t36137 - F::cast_from(0.64025200389650807212e-1_f64) * t36139 - t32923 - t36141 - F::cast_from(0.42874018118069736972e-2_f64) * t40442 + t37888 - t40446 / F::cast_from(64.0_f64) - t37892;
    t42110
}
