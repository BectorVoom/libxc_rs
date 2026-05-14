//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1100/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1100<F: Float>(t3431: F, t5891: F, t1165: F, t3451: F, t4183: F, t5852: F, t3372: F, t6157: F, t13092: F, t5932: F, t17550: F, t5928: F, t1180: F, t1181: F, t1531: F, t17205: F, t1889: F, t1894: F, t3396: F, t3462: F, t4463: F, t4684: F, t5094: F, t5111: F, t530: F, t6138: F, t8480: F, t945: F, t955: F) -> (F,) {
    let t22349 = t3431 * t5891;
    let t22369 = t3451 * t1165 * t5852 * t4183;
    let t22371 = t3372 * t6157;
    let t22378 = t13092 * t5932;
    let t22380 = t17550 * t5928;
    let t22382 = -0.17149607247227894789e-2 * t1180 * t8480 * t4684 - 0.80031500487063509014e-2 * t22349 - 0.21437009059034868486e-3 * t1180 * t1165 * t5852 * t955 + 0.34299214494455789578e-1 * t4463 * t1181 * t530 * t5111 - 0.17149607247227894789e-2 * t3462 * t1165 * t1889 * t945 + 0.85748036236139473944e-3 * t1531 * t1181 * t1894 * t945 - 0.42874018118069736972e-3 * t22369 - 0.20007875121765877254e-2 * t22371 + 0.20007875121765877254e-2 * t17205 - 0.20579528696673473747e-1 * t3396 * t1181 * t6138 * t5094 - 0.64025200389650807212e-1 * t22378 - 0.16006300097412701803e0 * t22380;
    (t22382,)
}
