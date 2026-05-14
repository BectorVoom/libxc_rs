//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 707/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk707<F: Float>(t10388: F, t799: F, t27: F, t89: F, t2740: F, t375: F, t10: F, t296: F, t3050: F, t1636: F, t825: F, t2660: F, t9571: F, t666: F, t835: F, t9592: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10389 = t799 * t10388;
    let t10391 = t89 * t27 * t10389;
    let t10394 = t89 * t375 * t2740;
    let t10397 = t10 * t3050 * t296;
    let t10398 = 14.0 / 81.0 * t10397;
    let t10400 = t89 * t1636 * t825;
    let t10402 = t2660 * t9571;
    let t10404 = t89 * t666 * t10402;
    let t10406 = t835 * t9592;
    (t10389, t10391, t10394, t10397, t10398, t10400, t10402, t10404, t10406)
}
