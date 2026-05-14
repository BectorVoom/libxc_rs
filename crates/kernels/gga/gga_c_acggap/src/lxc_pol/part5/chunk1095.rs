//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1095/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1095<F: Float>(t1180: F, t1181: F, t13451: F, t17068: F, t17070: F, t17072: F, t174: F, t19510: F, t22193: F, t22198: F, t22200: F, t22202: F, t22209: F, t22211: F, t22213: F, t3169: F, t387: F, t418: F, t422: F, t5862: F) -> (F,) {
    let t22215 = -0.85748036236139473944e-3 * t1180 * t1181 * t5862 * t3169 - 0.85748036236139473944e-3 * t22193 - 0.80031500487063509016e-2 * t17068 - 0.16006300097412701803e-1 * t17070 + 0.32012600194825403606e-1 * t17072 + 0.20007875121765877254e-2 * t22198 - 0.85748036236139473944e-3 * t22200 + 0.20007875121765877254e-2 * t22202 + t13451 - 0.85748036236139473944e-3 * t418 * t422 * t387 * t174 * t19510 + 0.40015750243531754508e-2 * t22209 + 0.80031500487063509014e-2 * t22211 + 0.80031500487063509014e-2 * t22213;
    (t22215,)
}
