//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1033/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1033<F: Float>(t9088: F, t9621: F, t9625: F, t9628: F, t9097: F, t9107: F, t9112: F, t9114: F, t9119: F, t7908: F, t7910: F, t8223: F, t8224: F, t8293: F, t8295: F, t8297: F) -> (F, F, F, F, F, F) {
    let t42559 = F::new(0.1702583995731913576e-4) * t9088;
    let t42560 = F::new(0.23948483403727617128e0) * t9621;
    let t42561 = F::new(0.23948483403727617128e0) * t9625;
    let t42562 = F::new(0.23948483403727617128e0) * t9628;
    let t42563 = F::new(0.5107751987195740728e-4) * t9097;
    let t42567 = F::new(0.5107751987195740728e-4) * t9107;
    let t42568 = F::new(0.1702583995731913576e-4) * t9112;
    let t42569 = F::new(0.1702583995731913576e-4) * t9114;
    let t42570 = F::new(0.638468998399467591e-4) * t9119;
    let t42571 = F::new(0.79453919800822633544e-4) * t7908 - F::new(0.23836175940246790064e-3) * t7910 - t8223 + t8224 + t8293 + t8295 + t8297 + t42567 + t42568 - t42569 - t42570;
    (t42559, t42560, t42561, t42562, t42563, t42571)
}
