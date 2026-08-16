//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1223/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1223(t1165: f64, t1180: f64, t1181: f64, t1531: f64, t17205: f64, t1889: f64, t1894: f64, t22349: f64, t22369: f64, t22371: f64, t22378: f64, t22380: f64, t3396: f64, t3462: f64, t4463: f64, t4684: f64, t5094: f64, t5111: f64, t530: f64, t5852: f64, t6138: f64, t8480: f64, t945: f64, t955: f64) -> f64 {
    let t22382 = -0.17149607247227894789e-2_f64 * t1180 * t8480 * t4684 - 0.80031500487063509014e-2_f64 * t22349 - 0.21437009059034868486e-3_f64 * t1180 * t1165 * t5852 * t955 + 0.34299214494455789578e-1_f64 * t4463 * t1181 * t530 * t5111 - 0.17149607247227894789e-2_f64 * t3462 * t1165 * t1889 * t945 + 0.85748036236139473944e-3_f64 * t1531 * t1181 * t1894 * t945 - 0.42874018118069736972e-3_f64 * t22369 - 0.20007875121765877254e-2_f64 * t22371 + 0.20007875121765877254e-2_f64 * t17205 - 0.20579528696673473747e-1_f64 * t3396 * t1181 * t6138 * t5094 - 0.64025200389650807212e-1_f64 * t22378 - 0.16006300097412701803e0_f64 * t22380;
    t22382
}
