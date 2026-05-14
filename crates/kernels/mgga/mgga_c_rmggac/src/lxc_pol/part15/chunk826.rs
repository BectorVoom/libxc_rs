//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 826/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk826<F: Float>(t352: F, t9888: F, t262: F, t36634: F, t10093: F, t495: F, t515: F, t7230: F, t7231: F, t10082: F, t3351: F, t7248: F, t1562: F, t8817: F, t31176: F, t681: F) -> (F, F, F, F, F, F, F) {
    let t45577 = t9888 * t352;
    let t45578 = t262 * t45577;
    let t45579 = t36634 * t45578;
    let t45584 = t7230 * t7231 * t515 * t10093 * t495;
    let t45589 = t3351 * t7248 * t515 * t10082 * t352;
    let t45591 = t1562 * t8817;
    let t45593 = t31176 * t681;
    (t45577, t45578, t45579, t45584, t45589, t45591, t45593)
}
