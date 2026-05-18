//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 786/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk786<F: Float>(t38314: F, t2145: F, t27: F, t3118: F, t570: F, t2046: F, t7297: F, t8482: F, t1341: F, t535: F, t638: F, t7310: F) -> (F, F, F, F) {
    let t38315 = F::new(0.19211284388664477842e-2) * t38314;
    let t38318 = t2145 * t27 * t3118 * t570;
    let t38322 = t2046 * t7297 * t8482;
    let t38326 = t638 * t7310 * t535 * t1341;
    (t38315, t38318, t38322, t38326)
}
