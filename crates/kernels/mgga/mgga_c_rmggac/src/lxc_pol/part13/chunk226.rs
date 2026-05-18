//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 226/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk226<F: Float>(t53: F, t60: F, t57: F, t912: F, t913: F, t916: F, t191: F, t284: F, t62: F, zeta_threshold: F) -> (F, F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t920 = piecewise3::<f64>(t54, F::new(0.0), F::new(4.0) / F::new(9.0) * t912 * t913 + F::new(4.0) / F::new(3.0) * t57 * t916);
    let t921 = F::new(1.0) / t191;
    let t922 = t284 * t284;
    let t925 = -t916;
    let t929 = piecewise3::<f64>(t61, F::new(0.0), F::new(4.0) / F::new(9.0) * t921 * t922 + F::new(4.0) / F::new(3.0) * t62 * t925);
    let t930 = t920 + t929;
    (t921, t922, t925, t930)
}
