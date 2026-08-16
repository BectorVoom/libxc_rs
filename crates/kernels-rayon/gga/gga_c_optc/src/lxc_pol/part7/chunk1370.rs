//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1370/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1370(t3126: f64, t9117: f64, t1: f64, t1111: f64, t1133: f64, t12004: f64, t26298: f64, t26981: f64, t27175: f64, t27196: f64, t27200: f64, t27202: f64, t27204: f64, t27209: f64, t27210: f64, t27215: f64, t27223: f64, t27227: f64, t27233: f64, t27237: f64, t3106: f64, t3107: f64, t3108: f64, t3245: f64, t450: f64, t8973: f64, t9175: f64) -> f64 {
    let t27244 = t9117 * t3126;
    let t27248 = 0.18110753103726578864e-2_f64 * t1133 * t27196 + 0.42074449172244793097e-1_f64 * t27200 + 0.56296038352410615326e5_f64 * t27202 * t450 * t27204 * t1 - 0.84444057528615922988e5_f64 * t27209 * t450 * t27210 * t1 + 0.3283935570557285894e5_f64 * t27215 * t450 * t27175 * t3107 * t1 - 0.43465807448943789272e-1_f64 * t1133 * t27223 + 0.28977204965962526181e-1_f64 * t27227 + t1111 * t3245 * t26298 / 8.0_f64 + 0.90553765518632894319e-2_f64 * t1133 * t27233 - 0.10866451862235947318e-1_f64 * t1133 * t27237 + 0.73258227843678641352e2_f64 * t8973 * t26981 * t3108 * t12004 + 0.63777043459628018514e5_f64 * t9175 * t3106 * t27244;
    t27248
}
