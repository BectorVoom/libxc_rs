//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1277/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1277<F: Float>(t3382: F, t6249: F, t6245: F, t14072: F, t14081: F, t14086: F, t14091: F, t14096: F, t14101: F, t14105: F, t14107: F, t14109: F, t14111: F) -> F {
    let t23618 = t3382 * t6249;
    let t23620 = t3382 * t6245;
    let t23630 = -F::cast_from(0.85748036236139473944e-3_f64) * t23618 + F::cast_from(0.85748036236139473944e-3_f64) * t23620 + F::cast_from(0.10289764348336736874e-1_f64) * t14072 - F::cast_from(0.51448821741683684367e-2_f64) * t14081 + F::cast_from(0.51448821741683684367e-2_f64) * t14086 - t14091 - F::cast_from(0.21437009059034868486e-3_f64) * t14096 + F::cast_from(0.25724410870841842183e-2_f64) * t14101 - t14105 - F::cast_from(0.13605355082800796533e0_f64) * t14107 + F::cast_from(0.68026775414003982664e-1_f64) * t14109 - F::cast_from(0.68026775414003982664e-1_f64) * t14111;
    t23630
}
