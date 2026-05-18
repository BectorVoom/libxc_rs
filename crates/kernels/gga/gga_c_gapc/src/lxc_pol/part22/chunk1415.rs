//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1415/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1415<F: Float>(t35173: F, t35184: F, t35186: F, t35188: F, t35197: F, t35200: F, t35177: F, t35182: F, t35190: F, t35192: F, t37223: F, t35205: F) -> (F, F) {
    let t37224 = F::new(0.63350674672043801542e-5) * t35173;
    let t37227 = F::new(0.69504740211613770836e-3) * t35184;
    let t37228 = F::new(0.34752370105806885418e-3) * t35186;
    let t37229 = F::new(0.34782544165564226085e-4) * t35188;
    let t37232 = F::new(0.12144921875e-2) * t35197;
    let t37233 = F::new(0.14232178796075385434e-7) * t35200;
    let t37234 = t37223 + t37224 - F::new(0.10925861285174334493e-8) * t35177 - F::new(0.38527756621470067412e-7) * t35182 - t37227 - t37228 + t37229 - F::new(0.31433990684987949195e-7) * t35190 - F::new(0.67632724766374884053e-5) * t35192 + t37232 - t37233;
    let t37236 = F::new(0.2748593934505475288e-5) * t35205;
    (t37234, t37236)
}
