//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1072/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1072<F: Float>(t34810: F, t37228: F, t42785: F, t44986: F, t44990: F, t44994: F, t44997: F, t45002: F, t45004: F, t45012: F, t45018: F, t45020: F, t45026: F, t45032: F, t45038: F, t45044: F, t45048: F) -> F {
    let t48324 = -F::new(0.30646511923174444368e-3) * t44986 + F::new(0.61293023846348888736e-3) * t44990 + F::new(0.15323255961587222184e-3) * t44994 - F::new(0.15323255961587222184e-3) * t44997 + F::new(0.638468998399467591e-4) * t45002 + F::new(0.5107751987195740728e-4) * t45004 + F::new(0.5107751987195740728e-4) * t45012 + F::new(0.5107751987195740728e-4) * t45018 + F::new(0.1702583995731913576e-4) * t45020 + F::new(0.1702583995731913576e-4) * t45026 + F::new(0.1702583995731913576e-4) * t45032 - t37228 - F::new(0.66671395154821946452e-1) * t34810 + F::new(0.85129199786595678799e-5) * t45038 - F::new(0.85129199786595678799e-5) * t45044 - F::new(0.85129199786595678799e-5) * t45048 - t42785;
    t48324
}
