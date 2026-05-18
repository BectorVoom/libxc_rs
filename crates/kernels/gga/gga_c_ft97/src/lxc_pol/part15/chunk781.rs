//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 781/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk781<F: Float>(t21181: F, t9717: F, t89: F, t9716: F, t2348: F, t666: F, t21204: F, t724: F, t446: F, t1131: F, t4965: F, t9744: F) -> (F, F, F, F, F, F, F, F) {
    let t21431 = t9717 * t21181;
    let t21433 = t89 * t9716 * t21431;
    let t21435 = t2348 * t21181;
    let t21437 = t89 * t666 * t21435;
    let t21439 = t724 * t21204;
    let t21440 = t446 * t21439;
    let t21442 = t4965 * t1131;
    let t21443 = t9744 * t21442;
    (t21431, t21433, t21435, t21437, t21439, t21440, t21442, t21443)
}
