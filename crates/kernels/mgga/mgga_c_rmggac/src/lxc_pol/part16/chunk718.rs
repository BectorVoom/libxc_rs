//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 718/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk718<F: Float>(t7487: F, t8346: F, t2145: F, t27: F, t3118: F, t570: F, t2046: F, t7297: F, t8482: F, t1341: F, t535: F, t638: F, t7310: F, t5542: F, t8601: F, t674: F) -> (F, F, F, F, F, F) {
    let t38314 = t7487 * t8346;
    let t38318 = t2145 * t27 * t3118 * t570;
    let t38322 = t2046 * t7297 * t8482;
    let t38326 = t638 * t7310 * t535 * t1341;
    let t38350 = t8601 * t5542;
    let t38351 = t38350 * t674;
    (t38314, t38318, t38322, t38326, t38350, t38351)
}
