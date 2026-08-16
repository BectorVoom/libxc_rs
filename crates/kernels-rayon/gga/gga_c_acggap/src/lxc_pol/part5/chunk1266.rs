//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1266/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1266(t1165: f64, t1173: f64, t1175: f64, t13958: f64, t13960: f64, t13962: f64, t13964: f64, t13966: f64, t17952: f64, t17962: f64, t20906: f64, t23351: f64, t23355: f64, t23359: f64, t23363: f64, t4706: f64, t5862: f64) -> f64 {
    let t23380 = 0.32012600194825403606e-1_f64 * t23351 - 0.17149607247227894789e-2_f64 * t23355 - 0.13719685797782315831e-1_f64 * t23359 + 0.85748036236139473944e-3_f64 * t23363 + 7.0_f64 / 18.0_f64 * t17952 + 0.30234122406223992295e0_f64 * t13958 - 0.15117061203111996148e0_f64 * t13960 + 0.15117061203111996148e0_f64 * t13962 + 0.11337795902333997111e-1_f64 * t13964 - 0.11337795902333997111e-1_f64 * t13966 - 0.13719685797782315831e-1_f64 * t17962 + 0.17149607247227894789e-2_f64 * t1173 * t1165 * t20906 * t1175 + 0.85748036236139473944e-3_f64 * t1173 * t1165 * t5862 * t4706;
    t23380
}
