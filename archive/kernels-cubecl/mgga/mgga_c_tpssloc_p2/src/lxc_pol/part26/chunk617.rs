//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 617/1384 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk617<F: Float>(t3565: F, t68: F, t484: F, t121: F, t486: F, t1216: F, t248: F, t1213: F, t478: F, t483: F, t3068: F, t1244: F) -> (F, F, F, F, F, F, F, F) {
    let t3566 = t3565 * t68;
    let t3567 = t3566 * t484;
    let t3570 = t121 * t486;
    let t3572 = t248 * t3570 * t1216;
    let t3573 = t1213 * t3572;
    let t3575 = t478 * t483;
    let t3576 = t3575 * t3068;
    let t3577 = t1244 * t3576;
    (t3566, t3567, t3570, t3572, t3573, t3575, t3576, t3577)
}
