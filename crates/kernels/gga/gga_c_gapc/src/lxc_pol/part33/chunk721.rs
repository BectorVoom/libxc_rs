//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 721/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk721<F: Float>(t1001: F, t8419: F, t8483: F, t8487: F, t8490: F, t8494: F, t8496: F, t8498: F, t8502: F, t8506: F, t8512: F, t8515: F) -> F {
    let t8517 = t8419 * t1001;
    let t8519 = F::new(0.16221005325193686047e-3) * t8483 - F::new(0.20855578275249024918e-2) * t8487 - F::new(0.17714874716515957771e-4) * t8490 + F::new(0.29524791194193262952e-5) * t8494 - F::new(0.28840947468194373793e-3) * t8496 + F::new(0.12360406057797588768e-3) * t8498 + F::new(0.772525378612349298e-5) * t8502 - F::new(0.45785004105758568397e-6) * t8506 - F::new(0.27721444647547803303e-5) * t8512 + F::new(0.6951859425083008306e-4) * t8515 - F::new(0.26319242435966565832e-3) * t8517;
    t8519
}
