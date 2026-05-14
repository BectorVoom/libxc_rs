//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 434/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk434<F: Float>(t2722: F, t454: F, t415: F, t2715: F, t2718: F, t504: F, t485: F, t488: F, t1340: F, t500: F) -> (F, F, F, F, F, F, F) {
    let t2723 = t454 * t2722;
    let t2724 = t415 * t2723;
    let t2726 = -0.10416666666666666667e-1 * t2715 * t2718 + 0.24872916666666666666e-2 * t2724;
    let t2727 = t2726 * t504;
    let t2728 = t485 * t488;
    let t2730 = t1340 * t500;
    let t2732 = t2728 / 16.0 - t2730 / 128.0;
    (t2723, t2724, t2726, t2727, t2728, t2730, t2732)
}
