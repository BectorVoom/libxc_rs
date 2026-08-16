//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 493/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk493<F: Float>(t3499: F, t3609: F, t3507: F, t491: F, t1932: F, t3508: F, t1215: F, t1235: F, t1246: F, t3493: F, t1209: F, t3032: F) -> (F, F, F, F, F, F) {
    let t3610 = t3499 * t3609;
    let t3611 = t491 * t3507;
    let t3612 = t1932 * t3508;
    let t3613 = t3611 * t3612;
    let t3616 = t1235 * t1215;
    let t3617 = t3616 * t1246;
    let t3620 = t491 * t3493;
    let t3621 = t3620 * t1246;
    let t3623 = t3032 * t1209;
    (t3610, t3611, t3613, t3617, t3621, t3623)
}
