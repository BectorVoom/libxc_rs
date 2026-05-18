//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 919/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk919<F: Float>(t10072: F, t7244: F, t5542: F, t9734: F, t7473: F, t7478: F, t9774: F, t1756: F, t1970: F, t1971: F, t209: F, t476: F, t515: F) -> (F, F, F, F, F, F) {
    let t45341 = t7244 * t10072;
    let t45343 = t9734 * t5542;
    let t45344 = t45343 * t7473;
    let t45345 = t45344 * t7478;
    let t45347 = t9774 * t5542;
    let t45348 = t45347 * t7473;
    let t45349 = t45348 * t7478;
    let t45355 = t1970 * t1971 * t515 * t1756 * t476 * t209;
    (t45341, t45343, t45345, t45347, t45349, t45355)
}
