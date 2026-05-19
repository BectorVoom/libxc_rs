//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1270/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1270<F: Float>(t1165: F, t3456: F, t4241: F, t5862: F, t1180: F, t1181: F, t13974: F, t13985: F, t14001: F, t14003: F, t14005: F, t14015: F, t14017: F, t14019: F, t18062: F, t18066: F, t5207: F, t5922: F) -> F {
    let t23454 = t3456 * t1165 * t5862 * t4241;
    let t23470 = -F::cast_from(0.85748036236139473944e-3_f64) * t23454 + F::cast_from(0.17149607247227894789e-2_f64) * t1180 * t1181 * t5922 * t5207 + F::cast_from(0.12004725073059526352e-1_f64) * t13974 - F::cast_from(0.85748036236139473944e-3_f64) * t13985 + F::cast_from(0.51448821741683684366e-2_f64) * t14001 - F::cast_from(0.25724410870841842183e-2_f64) * t14003 + F::cast_from(0.51448821741683684367e-2_f64) * t14005 - F::cast_from(0.34299214494455789578e-2_f64) * t18062 - F::cast_from(0.85748036236139473944e-3_f64) * t18066 - F::cast_from(0.51448821741683684367e-2_f64) * t14015 - F::cast_from(0.45351183609335988444e-1_f64) * t14017 + F::cast_from(0.68026775414003982664e-1_f64) * t14019;
    t23470
}
