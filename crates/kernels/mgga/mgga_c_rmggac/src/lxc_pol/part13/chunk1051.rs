//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1051/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1051<F: Float>(t39285: F, t39295: F, t39308: F, t39282: F, t39289: F, t39293: F, t39297: F, t39301: F, t39306: F, t39310: F, t39312: F, t39314: F, t39316: F, t39323: F, t39325: F, t4985: F, t6473: F, t8045: F, t8278: F) -> F {
    let t42906 = F::new(0.39726959900411316772e-4) * t39285;
    let t42909 = F::new(0.39726959900411316772e-4) * t39295;
    let t42913 = F::new(0.66211599834018861287e-4) * t39308;
    let t42924 = -F::new(0.212822999466489197e-4) * t39282 + t42906 - F::new(0.79453919800822633545e-4) * t39289 - F::new(0.5107751987195740728e-4) * t39293 + t42909 - F::new(0.85129199786595678799e-5) * t39297 + F::new(0.23942587439980034662e-4) * t39301 + F::new(0.1064114997332445985e-4) * t39306 - t42913 - F::new(0.85129199786595678799e-5) * t39310 + F::new(0.212822999466489197e-4) * t39312 - F::new(0.638468998399467591e-4) * t39314 + F::new(0.638468998399467591e-4) * t39316 - F::new(0.23948483403727617128e0) * t6473 * t8045 + F::new(0.59871208509319042821e-1) * t4985 * t8278 + F::new(0.1702583995731913576e-4) * t39323 - F::new(0.1702583995731913576e-4) * t39325;
    t42924
}
