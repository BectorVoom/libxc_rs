//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 763/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk763<F: Float>(t2580: F, t7245: F, t1841: F, t1897: F, t2504: F, t2508: F, t2509: F, t2577: F, t5227: F, t5288: F, t5293: F, t5524: F, t7129: F, t7137: F, t7204: F, t7207: F, t7212: F, t7215: F, t7223: F, t7228: F, t7233: F, t7236: F, t7239: F, t7242: F) -> F {
    let t7246 = t2580 * t7245;
    let t7249 = -F::new(0.20508069947045931424e-1) * t5293 * t2504 + F::new(0.20508069947045931424e-1) * t7137 * t2509 - F::new(0.15381052460284448567e-1) * t5288 * t2504 - F::new(0.23071578690426672851e-1) * t2508 * t7204 - F::new(0.85450291446024714264e-3) * t7207 + F::new(0.32043859292259267849e-3) * t7212 + F::new(0.64087718584518535698e-3) * t7215 - F::new(0.8545029144602471425e-3) * t5524 * t2577 + F::new(0.17090058289204942853e-2) * t5227 * t2577 + F::new(0.17090058289204942853e-2) * t1841 * t7223 - F::new(0.46143157380853345701e-1) * t2508 * t7228 + F::new(0.15381052460284448567e-1) * t7129 * t2509 - F::new(0.15381052460284448567e-1) * t1897 * t7233 - F::new(0.76905262301422242837e-2) * t1897 * t7236 + F::new(0.76905262301422242837e-2) * t2508 * t7239 + F::new(0.15381052460284448567e-1) * t2508 * t7242 + F::new(0.15381052460284448567e-1) * t2508 * t7246;
    t7249
}
