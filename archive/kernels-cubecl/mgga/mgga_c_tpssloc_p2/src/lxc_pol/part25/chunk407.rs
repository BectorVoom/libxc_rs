//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 407/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk407<F: Float>(t2223: F, t14: F, t21: F, t594: F, t598: F, t15: F) -> (F, F, F, F, F) {
    let t2224 = F::cast_from(16.0_f64) * t2223;
    let t2225 = t14 * t21;
    let t2226 = F::cast_from(0.778e2_f64) * t2225;
    let t2228 = F::cast_from(0.16272e3_f64) * t594 * t598;
    let t2229 = t15 * t15;
    (t2224, t2225, t2226, t2228, t2229)
}
