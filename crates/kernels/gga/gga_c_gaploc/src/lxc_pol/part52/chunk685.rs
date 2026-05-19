//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 685/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk685<F: Float>(t13276: F, t6320: F, t2268: F, t12798: F, t12383: F, t12386: F, t12392: F, t12395: F, t12397: F, t12400: F, t471: F, t12412: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13277 = t6320 * t13276;
    let t13279 = F::cast_from(0.17073003981405689759e0_f64) * t2268 * t13277;
    let t13280 = F::cast_from(0.47425011059460249332e-2_f64) * t12798;
    let t13287 = -F::new(3.0) / F::new(128.0) * t12383 - F::new(27.0) / F::new(4096.0) * t12386 + F::new(27.0) / F::new(262144.0) * t12392 - F::new(9.0) / F::new(262144.0) * t12395 + F::new(9.0) / F::new(4096.0) * t12397 + t12400 / F::new(128.0);
    let t13288 = t13287 * t471;
    let t13291 = F::new(9.0) / F::new(128.0) * t12383;
    let t13292 = F::new(9.0) / F::new(4096.0) * t12386;
    let t13293 = F::new(3.0) / F::new(4096.0) * t12397;
    let t13294 = F::new(3.0) / F::new(128.0) * t12400;
    let t13295 = F::new(4.0) * t12412;
    (t13277, t13279, t13280, t13287, t13288, t13291, t13292, t13293, t13294, t13295)
}
