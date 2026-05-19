//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 323/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk323<F: Float>(t1138: F, t1141: F, t1145: F, t1150: F, t1152: F, t1156: F, t1168: F, t1173: F, t1177: F, t1180: F, t1184: F, t1190: F, t1195: F, t1200: F, t1205: F, t335: F, t367: F, t418: F) -> F {
    let t1208 = F::new(7.0) / F::new(72.0) * t1138 + F::new(7.0) / F::new(144.0) * t1141 - t335 * t1145 / F::new(24.0) + t1150 * t1152 / F::new(16.0) + t367 * t1156 / F::new(48.0) + F::cast_from(0.42874018118069736972e-3_f64) * t1168 + F::cast_from(0.17149607247227894789e-2_f64) * t1173 * t1177 - F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t1184 + F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t1190 - F::cast_from(0.85748036236139473944e-3_f64) * t418 * t1195 + F::cast_from(0.42874018118069736972e-3_f64) * t418 * t1200 - F::cast_from(0.42874018118069736972e-3_f64) * t418 * t1205;
    t1208
}
