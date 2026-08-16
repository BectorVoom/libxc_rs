//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1223/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1223<F: Float>(t1165: F, t1180: F, t1181: F, t1531: F, t17205: F, t1889: F, t1894: F, t22349: F, t22369: F, t22371: F, t22378: F, t22380: F, t3396: F, t3462: F, t4463: F, t4684: F, t5094: F, t5111: F, t530: F, t5852: F, t6138: F, t8480: F, t945: F, t955: F) -> F {
    let t22382 = -F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t8480 * t4684 - F::cast_from(0.80031500487063509014e-2_f64) * t22349 - F::cast_from(0.21437009059034868486e-3_f64) * t1180 * t1165 * t5852 * t955 + F::cast_from(0.34299214494455789578e-1_f64) * t4463 * t1181 * t530 * t5111 - F::cast_from(0.17149607247227894789e-2_f64) * t3462 * t1165 * t1889 * t945 + F::cast_from(0.85748036236139473944e-3_f64) * t1531 * t1181 * t1894 * t945 - F::cast_from(0.42874018118069736972e-3_f64) * t22369 - F::cast_from(0.20007875121765877254e-2_f64) * t22371 + F::cast_from(0.20007875121765877254e-2_f64) * t17205 - F::cast_from(0.20579528696673473747e-1_f64) * t3396 * t1181 * t6138 * t5094 - F::cast_from(0.64025200389650807212e-1_f64) * t22378 - F::cast_from(0.16006300097412701803e0_f64) * t22380;
    t22382
}
