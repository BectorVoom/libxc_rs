//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1078/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1078<F: Float>(t3575: F, t6500: F, t3952: F, t3579: F, t1312: F, t4374: F, t442: F, t2059: F, t4376: F, t19033: F, t4391: F, t6449: F, t1308: F, t3969: F, t6458: F, t4384: F, t6459: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t21643 = t6500 * t3575;
    let t21644 = t3952 * t21643;
    let t21647 = t6500 * t3579;
    let t21648 = t1312 * t21647;
    let t21651 = t4374 * t442;
    let t21652 = t2059 * t4376;
    let t21653 = t21651 * t21652;
    let t21654 = t1312 * t21653;
    let t21657 = t4391 * t19033;
    let t21658 = t1312 * t21657;
    let t21661 = t6449 * sigma0;
    let t21662 = t21661 * t1308;
    let t21665 = t6458 * t3969;
    let t21668 = t6459 * t4384;
    (t21644, t21648, t21652, t21654, t21658, t21662, t21665, t21668)
}
