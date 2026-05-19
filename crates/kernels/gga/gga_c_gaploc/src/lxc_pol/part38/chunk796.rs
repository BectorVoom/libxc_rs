//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 796/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk796<F: Float>(t12987: F, t7014: F, t2365: F, t31558: F, t7025: F, t12943: F, t4379: F, t40452: F, t10608: F, t9272: F, t9278: F, t34600: F, t544: F, t9287: F) -> (F, F, F, F, F, F) {
    let t42256 = t7014 * t12987;
    let t42259 = t7025 * t2365 * t31558;
    let t42316 = t4379 * t12943;
    let t42341 = F::cast_from(0.31952438294933958063e0_f64) * t40452;
    let t42349 = t9272 * t10608 * t9278;
    let t42366 = t544 * t34600 * t9287;
    (t42256, t42259, t42316, t42341, t42349, t42366)
}
