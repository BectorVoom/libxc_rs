//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1136/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1136<F: Float>(t3431: F, t5717: F, t1163: F, t1165: F, t4298: F, t6403: F, t1181: F, t22040: F, t3361: F, t4643: F, t5122: F, t5852: F, t1173: F, t1175: F, t13958: F, t13960: F, t13962: F, t13964: F, t13966: F, t17952: F, t17962: F, t20906: F, t4706: F, t5862: F) -> (F,) {
    let t23351 = t3431 * t5717;
    let t23355 = t1163 * t1165 * t4298 * t6403;
    let t23359 = t3361 * t1181 * t4643 * t22040;
    let t23363 = t1163 * t1181 * t5852 * t5122;
    let t23380 = 0.32012600194825403606e-1 * t23351 - 0.17149607247227894789e-2 * t23355 - 0.13719685797782315831e-1 * t23359 + 0.85748036236139473944e-3 * t23363 + 7.0 / 18.0 * t17952 + 0.30234122406223992295e0 * t13958 - 0.15117061203111996148e0 * t13960 + 0.15117061203111996148e0 * t13962 + 0.11337795902333997111e-1 * t13964 - 0.11337795902333997111e-1 * t13966 - 0.13719685797782315831e-1 * t17962 + 0.17149607247227894789e-2 * t1173 * t1165 * t20906 * t1175 + 0.85748036236139473944e-3 * t1173 * t1165 * t5862 * t4706;
    (t23380,)
}
