//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 830/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk830<F: Float>(t2007: F, t45561: F, t321: F, t9888: F, t262: F, t36629: F, t333: F, t41634: F, t352: F, t36634: F, t10093: F, t495: F, t515: F, t7230: F, t7231: F, t10082: F, t3351: F, t7248: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t45562 = t45561 * t2007;
    let t45568 = t9888 * t321;
    let t45569 = t262 * t45568;
    let t45570 = t36629 * t45569;
    let t45572 = t9888 * t333;
    let t45573 = t262 * t45572;
    let t45574 = t41634 * t45573;
    let t45577 = t9888 * t352;
    let t45578 = t262 * t45577;
    let t45579 = t36634 * t45578;
    let t45584 = t7230 * t7231 * t515 * t10093 * t495;
    let t45589 = t3351 * t7248 * t515 * t10082 * t352;
    (t45562, t45568, t45569, t45570, t45572, t45573, t45574, t45577, t45578, t45579, t45584, t45589)
}
