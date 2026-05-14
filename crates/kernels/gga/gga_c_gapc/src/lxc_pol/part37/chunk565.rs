//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 565/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk565<F: Float>(t3077: F, t3082: F, t3086: F, t3089: F, t3092: F, t3098: F, t3101: F, t3106: F, t3110: F, t3118: F, t3124: F, t3129: F, t3134: F, t3145: F, t3147: F, t3150: F, t3158: F, t3161: F, t3164: F, t3166: F, t3168: F, t3173: F, t3175: F) -> (F, F) {
    let t3522 = -0.10005749997240850277e-7 * t3077 - 0.6747178819444444445e-5 * t3082 - 0.20241536458333333335e-4 * t3086 - 0.17376185052903442709e-3 * t3089 - 0.17376185052903442709e-3 * t3092 + 0.14480154210752868924e-5 * t3098 - 0.2318836277704281739e-4 * t3101 - 0.27801896084645508334e-2 * t3106 - 0.27801896084645508334e-2 * t3110 + 0.56275309320814680969e-8 * t3118 + 0.5627530932081468097e-7 * t3124;
    let t3535 = -0.75883739738679928909e-7 * t3129 + 0.1349212892553729136e-6 * t3134 - 0.49240895655712845849e-7 * t3145 + 0.27801896084645508334e-2 * t3147 + 0.20241536458333333335e-4 * t3150 + 0.29518907335069444447e-5 * t3158 + 0.27801896084645508334e-2 * t3161 - 0.28985453471303521736e-5 * t3164 - 0.10120768229166666668e-3 * t3166 + 0.12380568050579229813e-5 * t3168 - 0.69504740211613770835e-4 * t3173 - 0.64871090864172852779e-2 * t3175;
    (t3522, t3535)
}
