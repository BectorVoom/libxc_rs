//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1144/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1144<F: Float>(t36913: F, t36916: F, t36922: F, t36925: F, t36936: F, t36948: F, t38123: F, t44004: F, t44008: F, t47840: F, t47845: F, t47855: F, t47857: F, t47861: F, t47866: F, t47868: F, t47872: F) -> F {
    let t49725 = t44004 - F::new(0.638468998399467591e-4) * t47840 + F::new(0.1915406995198402773e-3) * t47845 - t44008 + F::new(0.72042316457491791901e-3) * t36913 + F::new(0.66211599834018861287e-4) * t36916 - F::new(0.38422568777328955681e-2) * t36922 - F::new(0.1440846329149835838e-2) * t36925 - F::new(0.72042316457491791901e-3) * t36936 + t38123 + F::new(0.20496175532535769483e-3) * t36948 - F::new(0.85129199786595678799e-5) * t47855 + F::new(0.2553875993597870364e-4) * t47857 + F::new(0.2553875993597870364e-4) * t47861 + F::new(0.1702583995731913576e-4) * t47866 - F::new(0.85129199786595678799e-5) * t47868 - F::new(0.212822999466489197e-4) * t47872;
    t49725
}
