//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1216/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1216<F: Float>(t1891: F, t3237: F, t5690: F, t997: F, t1180: F, t1181: F, t13451: F, t17068: F, t17070: F, t17072: F, t174: F, t19510: F, t22193: F, t22198: F, t22200: F, t22202: F, t22209: F, t3169: F, t387: F, t418: F, t422: F, t5862: F) -> F {
    let t22211 = t3237 * t1891;
    let t22213 = t997 * t5690;
    let t22215 = -F::cast_from(0.85748036236139473944e-3_f64) * t1180 * t1181 * t5862 * t3169 - F::cast_from(0.85748036236139473944e-3_f64) * t22193 - F::cast_from(0.80031500487063509016e-2_f64) * t17068 - F::cast_from(0.16006300097412701803e-1_f64) * t17070 + F::cast_from(0.32012600194825403606e-1_f64) * t17072 + F::cast_from(0.20007875121765877254e-2_f64) * t22198 - F::cast_from(0.85748036236139473944e-3_f64) * t22200 + F::cast_from(0.20007875121765877254e-2_f64) * t22202 + t13451 - F::cast_from(0.85748036236139473944e-3_f64) * t418 * t422 * t387 * t174 * t19510 + F::cast_from(0.40015750243531754508e-2_f64) * t22209 + F::cast_from(0.80031500487063509014e-2_f64) * t22211 + F::cast_from(0.80031500487063509014e-2_f64) * t22213;
    t22215
}
