//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 594/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk594<F: Float>(t3398: F, t3400: F, t3409: F, t3416: F, t3419: F, t3422: F, t3425: F, t3428: F, t3432: F, t3435: F, t3441: F, t3445: F) -> F {
    let t3620 = -F::new(0.27801896084645508334e-2) * t3398 + F::new(0.10821235962619981449e-3) * t3400 + F::new(0.84410248952307505288e-7) * t3409 + F::new(0.16882049790461501058e-6) * t3416 - F::new(0.75883739738679928909e-7) * t3419 + F::new(0.1349212892553729136e-6) * t3422 - F::new(0.17376185052903442709e-3) * t3425 - F::new(0.17376185052903442709e-3) * t3428 + F::new(0.14480154210752868924e-5) * t3432 - F::new(0.2318836277704281739e-4) * t3435 + F::new(0.28136749650769168429e-8) * t3441 - F::new(0.69504740211613770835e-4) * t3445;
    t3620
}
