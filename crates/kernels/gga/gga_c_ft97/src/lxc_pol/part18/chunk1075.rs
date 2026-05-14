//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1075/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1075<F: Float>(t64665: F, t64701: F, t1609: F, t51: F, t358: F, t497: F, t1326: F, t8417: F, t1851: F, t5704: F, t22892: F, t5495: F, t22883: F, t378: F, t23246: F, t8392: F) -> (F, F, F, F, F, F, F, F) {
    let t64702 = t64665 + t64701;
    let t65750 = t51 * t1609;
    let t91480 = t497 * t358;
    let t91493 = t1326 * t8417;
    let t91496 = t5704 * t1851;
    let t91501 = t5495 * t22892;
    let t91504 = t378 * t22883;
    let t91523 = t8392 * t23246;
    (t64702, t65750, t91480, t91493, t91496, t91501, t91504, t91523)
}
