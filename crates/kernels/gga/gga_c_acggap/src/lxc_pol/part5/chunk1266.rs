//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1266/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1266<F: Float>(t1165: F, t1173: F, t1175: F, t13958: F, t13960: F, t13962: F, t13964: F, t13966: F, t17952: F, t17962: F, t20906: F, t23351: F, t23355: F, t23359: F, t23363: F, t4706: F, t5862: F) -> F {
    let t23380 = F::cast_from(0.32012600194825403606e-1_f64) * t23351 - F::cast_from(0.17149607247227894789e-2_f64) * t23355 - F::cast_from(0.13719685797782315831e-1_f64) * t23359 + F::cast_from(0.85748036236139473944e-3_f64) * t23363 + F::new(7.0) / F::new(18.0) * t17952 + F::cast_from(0.30234122406223992295e0_f64) * t13958 - F::cast_from(0.15117061203111996148e0_f64) * t13960 + F::cast_from(0.15117061203111996148e0_f64) * t13962 + F::cast_from(0.11337795902333997111e-1_f64) * t13964 - F::cast_from(0.11337795902333997111e-1_f64) * t13966 - F::cast_from(0.13719685797782315831e-1_f64) * t17962 + F::cast_from(0.17149607247227894789e-2_f64) * t1173 * t1165 * t20906 * t1175 + F::cast_from(0.85748036236139473944e-3_f64) * t1173 * t1165 * t5862 * t4706;
    t23380
}
