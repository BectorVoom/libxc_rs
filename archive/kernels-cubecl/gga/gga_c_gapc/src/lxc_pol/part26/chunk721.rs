//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 721/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk721<F: Float>(t1001: F, t8419: F, t8483: F, t8487: F, t8490: F, t8494: F, t8496: F, t8498: F, t8502: F, t8506: F, t8512: F, t8515: F) -> F {
    let t8517 = t8419 * t1001;
    let t8519 = F::cast_from(0.16221005325193686047e-3_f64) * t8483 - F::cast_from(0.20855578275249024918e-2_f64) * t8487 - F::cast_from(0.17714874716515957771e-4_f64) * t8490 + F::cast_from(0.29524791194193262952e-5_f64) * t8494 - F::cast_from(0.28840947468194373793e-3_f64) * t8496 + F::cast_from(0.12360406057797588768e-3_f64) * t8498 + F::cast_from(0.772525378612349298e-5_f64) * t8502 - F::cast_from(0.45785004105758568397e-6_f64) * t8506 - F::cast_from(0.27721444647547803303e-5_f64) * t8512 + F::cast_from(0.6951859425083008306e-4_f64) * t8515 - F::cast_from(0.26319242435966565832e-3_f64) * t8517;
    t8519
}
