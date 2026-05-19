//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1370/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1370<F: Float>(t3126: F, t9117: F, t1: F, t1111: F, t1133: F, t12004: F, t26298: F, t26981: F, t27175: F, t27196: F, t27200: F, t27202: F, t27204: F, t27209: F, t27210: F, t27215: F, t27223: F, t27227: F, t27233: F, t27237: F, t3106: F, t3107: F, t3108: F, t3245: F, t450: F, t8973: F, t9175: F) -> F {
    let t27244 = t9117 * t3126;
    let t27248 = F::cast_from(0.18110753103726578864e-2_f64) * t1133 * t27196 + F::cast_from(0.42074449172244793097e-1_f64) * t27200 + F::cast_from(0.56296038352410615326e5_f64) * t27202 * t450 * t27204 * t1 - F::cast_from(0.84444057528615922988e5_f64) * t27209 * t450 * t27210 * t1 + F::cast_from(0.3283935570557285894e5_f64) * t27215 * t450 * t27175 * t3107 * t1 - F::cast_from(0.43465807448943789272e-1_f64) * t1133 * t27223 + F::cast_from(0.28977204965962526181e-1_f64) * t27227 + t1111 * t3245 * t26298 / F::new(8.0) + F::cast_from(0.90553765518632894319e-2_f64) * t1133 * t27233 - F::cast_from(0.10866451862235947318e-1_f64) * t1133 * t27237 + F::cast_from(0.73258227843678641352e2_f64) * t8973 * t26981 * t3108 * t12004 + F::cast_from(0.63777043459628018514e5_f64) * t9175 * t3106 * t27244;
    t27248
}
