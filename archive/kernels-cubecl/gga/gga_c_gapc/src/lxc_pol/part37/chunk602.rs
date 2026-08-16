//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 602/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk602<F: Float>(t3029: F, t3032: F, t3037: F, t3040: F, t3047: F, t3049: F, t3051: F, t3054: F, t3058: F, t3062: F, t3066: F, t3077: F, t3082: F, t3086: F, t3089: F, t3092: F, t3098: F, t3101: F, t3106: F, t3110: F, t3118: F, t3124: F) -> (F, F) {
    let t3509 = F::cast_from(0.14492726735651760868e-5_f64) * t3029 + F::cast_from(0.12357942809624928455e-3_f64) * t3032 - F::cast_from(0.25745714186718600948e-5_f64) * t3037 + F::cast_from(0.2318836277704281739e-4_f64) * t3040 + F::cast_from(0.21135226489492151266e-6_f64) * t3047 - F::cast_from(0.4637672555408563478e-4_f64) * t3049 + F::cast_from(0.4637672555408563478e-4_f64) * t3051 + F::cast_from(0.38647271295071362317e-6_f64) * t3054 - F::cast_from(0.68714848362636882201e-6_f64) * t3058 - F::cast_from(0.16882592796244404291e-6_f64) * t3062 - F::cast_from(0.16882592796244404291e-6_f64) * t3066;
    let t3522 = -F::cast_from(0.10005749997240850277e-7_f64) * t3077 - F::cast_from(0.6747178819444444445e-5_f64) * t3082 - F::cast_from(0.20241536458333333335e-4_f64) * t3086 - F::cast_from(0.17376185052903442709e-3_f64) * t3089 - F::cast_from(0.17376185052903442709e-3_f64) * t3092 + F::cast_from(0.14480154210752868924e-5_f64) * t3098 - F::cast_from(0.2318836277704281739e-4_f64) * t3101 - F::cast_from(0.27801896084645508334e-2_f64) * t3106 - F::cast_from(0.27801896084645508334e-2_f64) * t3110 + F::cast_from(0.56275309320814680969e-8_f64) * t3118 + F::cast_from(0.5627530932081468097e-7_f64) * t3124;
    (t3509, t3522)
}
