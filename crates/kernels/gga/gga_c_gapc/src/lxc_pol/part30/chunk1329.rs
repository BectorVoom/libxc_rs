//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1329/1331 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1329<F: Float>(t38063: F, t38066: F, t38069: F, t38070: F, t38073: F, t38075: F, t38077: F, t38082: F, t38086: F, t38088: F, t38093: F, t38503: F, t38508: F, t38514: F, t38520: F) -> F {
    let t38860 = t38063 + t38066 - t38069 - t38070 - t38073 + t38075 - t38077 - t38082 - t38086 - t38088 + t38093 - t38503 + t38508 + t38514 + t38520;
    t38860
}
