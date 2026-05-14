//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 726/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk726<F: Float>(t534: F, t6449: F, t1571: F, t2317: F, t1576: F, t2318: F, t1308: F, t1583: F, t1593: F, t2322: F, t4385: F, t4397: F, t4411: F, t4418: F, t4421: F, t4505: F, t541: F, sigma0: F) -> (F, F, F, F, F) {
    let t6450 = t6449 * t534;
    let t6453 = t2317 * t1571;
    let t6456 = t2318 * t1576;
    let t6458 = t2317 * sigma0;
    let t6459 = t6458 * t1308;
    let t6470 = 0.2698618307426597582e-1 * t6450 * t541 - 0.71963154864709268853e-1 * t6453 * t541 + 0.89953943580886586067e-2 * t6456 + 0.89953943580886586067e-2 * t6459 * t1583 - 0.2698618307426597582e-1 * t2318 * t1593 + 0.89953943580886586067e-2 * t4505 - 0.23987718288236422951e-1 * t4411 - t4418 + 0.29984647860295528689e-2 * t4385 - 0.89953943580886586067e-2 * t4421 + 0.89953943580886586067e-2 * t4397 * t2322;
    (t6450, t6453, t6458, t6459, t6470)
}
