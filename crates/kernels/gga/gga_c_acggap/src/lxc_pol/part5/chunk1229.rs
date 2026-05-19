//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1229/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1229<F: Float>(t14345: F, t1817: F, t1008: F, t6205: F, t3431: F, t6255: F, t6260: F, t3382: F, t5864: F, t12752: F, t1750: F, t1089: F, t1180: F, t1181: F, t1182: F, t17308: F, t17310: F, t1815: F, t20906: F, t301: F, t368: F, t398: F, t418: F, t429: F, t5784: F, t5964: F, t942: F, t966: F) -> F {
    let t22507 = t14345 * t1817;
    let t22509 = t1008 * t6205;
    let t22511 = t3431 * t6255;
    let t22513 = t3431 * t6260;
    let t22515 = t3382 * t5864;
    let t22522 = t12752 * t1750;
    let t22524 = F::cast_from(0.85748036236139473944e-3_f64) * t942 * t398 * t966 * t1815 - F::cast_from(0.34299214494455789578e-2_f64) * t418 * t1089 * t368 * t5784 * t301 - F::new(7.0) / F::new(36.0) * t17308 - F::cast_from(0.68598428988911579156e-2_f64) * t418 * t1089 * t429 * t5964 + F::cast_from(0.42874018118069736972e-3_f64) * t22507 - F::cast_from(0.34299214494455789578e-2_f64) * t22509 - F::cast_from(0.16006300097412701803e-1_f64) * t22511 - F::cast_from(0.16006300097412701803e-1_f64) * t22513 - F::cast_from(0.85748036236139473944e-3_f64) * t22515 - F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t1181 * t20906 * t1182 - F::cast_from(0.40015750243531754508e-2_f64) * t17310 - F::cast_from(0.16006300097412701803e-1_f64) * t22522;
    t22524
}
