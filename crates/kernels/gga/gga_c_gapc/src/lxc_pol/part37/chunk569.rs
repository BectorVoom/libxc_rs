//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 569/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk569<F: Float>(t3355: F, t3358: F, t3361: F, t3365: F, t3369: F, t3372: F, t3376: F, t3380: F, t3385: F, t3389: F, t3393: F, t3398: F, t3400: F, t3409: F, t3416: F, t3419: F, t3422: F, t3425: F, t3428: F, t3432: F, t3435: F, t3441: F, t3445: F) -> (F, F) {
    let t3607 = 0.4637672555408563478e-4 * t3355 + 0.38647271295071362317e-6 * t3358 - 0.68714848362636882201e-6 * t3361 - 0.84410248952307505288e-7 * t3365 - 0.84410248952307505288e-7 * t3369 + 0.61900849231692170545e-6 * t3372 + 0.28136749650769168429e-7 * t3376 - 0.27801896084645508334e-2 * t3380 + 0.12163329537032409896e-2 * t3385 - 0.10120442708333333334e-4 * t3389 - 0.10120442708333333334e-4 * t3393;
    let t3620 = -0.27801896084645508334e-2 * t3398 + 0.10821235962619981449e-3 * t3400 + 0.84410248952307505288e-7 * t3409 + 0.16882049790461501058e-6 * t3416 - 0.75883739738679928909e-7 * t3419 + 0.1349212892553729136e-6 * t3422 - 0.17376185052903442709e-3 * t3425 - 0.17376185052903442709e-3 * t3428 + 0.14480154210752868924e-5 * t3432 - 0.2318836277704281739e-4 * t3435 + 0.28136749650769168429e-8 * t3441 - 0.69504740211613770835e-4 * t3445;
    (t3607, t3620)
}
