//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1227/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1227<F: Float>(t1308: F, t388: F, t41167: F, t32105: F, t9439: F, t18681: F, t2715: F, t2717: F, t55867: F, t9445: F, t1328: F, t13830: F, t41006: F, t9422: F, t9434: F, t32069: F, t3936: F) -> (F, F, F, F, F, F, F, F, F) {
    let t110474 = t41167 * t388 * t1308;
    let t110505 = t9439 * t32105;
    let t110509 = 0.38580246913580246915e-2 * t2715 * t18681 * t2717;
    let t110524 = t9445 * t55867;
    let t110558 = t13830 * t1328;
    let t110566 = t41006 * t388 * t1308;
    let t110593 = t9422 * t32105;
    let t110595 = t9434 * t32105;
    let t110605 = t3936 * t32069;
    (t110474, t110505, t110509, t110524, t110558, t110566, t110593, t110595, t110605)
}
