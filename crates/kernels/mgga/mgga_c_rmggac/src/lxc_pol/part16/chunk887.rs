//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 887/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk887<F: Float>(t40731: F, t8571: F, t1981: F, t632: F, t676: F, t8512: F, t39300: F, t40246: F, t1971: F, t236: F, t35331: F, t6135: F, t6139: F, t7365: F, t2157: F, t6349: F) -> (F, F, F, F, F, F) {
    let t47219 = t8571 * t40731;
    let t47223 = t8512 * t1981 * t676 * t632;
    let t47225 = t39300 * t40246;
    let t47229 = t35331 * t1971 * t236 * t6135;
    let t47233 = t7365 * t1971 * t236 * t6139;
    let t47235 = t6349 * t2157;
    (t47219, t47223, t47225, t47229, t47233, t47235)
}
