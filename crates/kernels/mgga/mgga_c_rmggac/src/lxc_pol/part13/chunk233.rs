//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 233/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk233<F: Float>(t53: F, t60: F, t417: F, t977: F, t978: F, t431: F, t58: F, t437: F, t913: F, t916: F, t63: F, t441: F, t922: F, t925: F, zeta_threshold: F) -> (F, F, F, F, F, F) {
    let t54 = t53 <= zeta_threshold;
    let t61 = t60 <= zeta_threshold;
    let t980 = t977 * t978 * t417;
    let t982 = F::new(0.11696447245269292414e1) * t431 * t980;
    let t983 = F::new(1.0) / t58;
    let t989 = piecewise3::<f64>(t54, F::new(0.0), -F::new(2.0) / F::new(9.0) * t983 * t913 + F::new(2.0) / F::new(3.0) * t437 * t916);
    let t990 = F::new(1.0) / t63;
    let t996 = piecewise3::<f64>(t61, F::new(0.0), -F::new(2.0) / F::new(9.0) * t990 * t922 + F::new(2.0) / F::new(3.0) * t441 * t925);
    (t980, t982, t983, t989, t990, t996)
}
