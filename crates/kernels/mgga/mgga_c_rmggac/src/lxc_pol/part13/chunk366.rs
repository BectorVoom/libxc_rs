//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 366/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk366<F: Float>(t2065: F, t2086: F, t321: F, t699: F, t305: F, t333: F, t326: F, t698: F, t874: F) -> (F, F, F, F, F, F, F) {
    let t2200 = F::cast_from(0.79828278012425390427e-1_f64) * t2065;
    let t2204 = F::cast_from(0.18183107769496894487e-1_f64) * t2086;
    let t2205 = t699 * t321;
    let t2206 = t305 * t2205;
    let t2208 = t699 * t333;
    let t2209 = t326 * t2208;
    let t2211 = t874 * t698;
    (t2200, t2204, t2205, t2206, t2208, t2209, t2211)
}
