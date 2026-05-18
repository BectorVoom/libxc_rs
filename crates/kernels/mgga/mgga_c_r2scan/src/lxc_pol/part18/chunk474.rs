//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 474/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk474<F: Float>(t2452: F, t41: F, t406: F, t899: F, t2267: F, t910: F, t2266: F, t879: F, t955: F, t1776: F, t1782: F) -> (F, F, F, F, F, F) {
    let t2453 = t41 * t2452;
    let t2454 = t406 * t899;
    let t2455 = F::new(4.0) * t2454;
    let t2456 = t2267 * t910;
    let t2457 = t2266 * t2456;
    let t2458 = F::new(3.0) * t2457;
    let t2460 = t879 * t955;
    let t2461 = t1776 - t1782;
    (t2453, t2454, t2455, t2458, t2460, t2461)
}
