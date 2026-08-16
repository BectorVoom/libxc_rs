//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 919/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk919<F: Float>(t39570: F, t8636: F, t39705: F, t8902: F, t17859: F, t9213: F, t9218: F, t1907: F, t1971: F, t333: F, t511: F, t7230: F) -> (F, F, F, F, F) {
    let t45289 = t39570 * t8636;
    let t45291 = t39705 * t8902;
    let t45293 = t17859 * t9213;
    let t45295 = t17859 * t9218;
    let t45300 = t7230 * t1971 * t511 * t1907 * t333;
    (t45289, t45291, t45293, t45295, t45300)
}
