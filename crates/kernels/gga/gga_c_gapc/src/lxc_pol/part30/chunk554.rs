//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 554/1135 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk554<F: Float>(t3029: F, t3032: F, t3037: F, t3040: F, t3047: F, t3049: F, t3051: F, t3054: F, t3058: F, t3062: F, t3066: F, t3077: F, t3082: F, t3086: F, t3089: F, t3092: F, t3098: F, t3101: F, t3106: F, t3110: F, t3118: F, t3124: F) -> (F, F) {
    let t3509 = 0.14492726735651760868e-5 * t3029 + 0.12357942809624928455e-3 * t3032 - 0.25745714186718600948e-5 * t3037 + 0.2318836277704281739e-4 * t3040 + 0.21135226489492151266e-6 * t3047 - 0.4637672555408563478e-4 * t3049 + 0.4637672555408563478e-4 * t3051 + 0.38647271295071362317e-6 * t3054 - 0.68714848362636882201e-6 * t3058 - 0.16882592796244404291e-6 * t3062 - 0.16882592796244404291e-6 * t3066;
    let t3522 = -0.10005749997240850277e-7 * t3077 - 0.6747178819444444445e-5 * t3082 - 0.20241536458333333335e-4 * t3086 - 0.17376185052903442709e-3 * t3089 - 0.17376185052903442709e-3 * t3092 + 0.14480154210752868924e-5 * t3098 - 0.2318836277704281739e-4 * t3101 - 0.27801896084645508334e-2 * t3106 - 0.27801896084645508334e-2 * t3110 + 0.56275309320814680969e-8 * t3118 + 0.5627530932081468097e-7 * t3124;
    (t3509, t3522)
}
