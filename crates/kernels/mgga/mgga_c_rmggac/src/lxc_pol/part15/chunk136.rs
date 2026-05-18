//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 136/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk136<F: Float>(t60: F, t284: F, t441: F, t440: F, zeta_threshold: F) -> F {
    let t61 = t60 <= zeta_threshold;
    let t444 = piecewise3::<f64>(t61, F::new(0.0), F::new(2.0) / F::new(3.0) * t441 * t284);
    let t446 = t440 / F::new(2.0) + t444 / F::new(2.0);
    t446
}
