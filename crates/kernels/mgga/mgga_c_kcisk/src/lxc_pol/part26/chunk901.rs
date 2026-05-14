//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 901/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk901<F: Float>(t19483: F, t19102: F, t3725: F, t5788: F, t2285: F, t4435: F, t2097: F, t3696: F, t1576: F, t6453: F, t2318: F, t4416: F, t6450: F, t20160: F, t6506: F, t1580: F) -> (F, F, F, F, F, F, F, F, F) {
    let t21771 = 0.27785333333333333334e0 * t19483;
    let t21804 = 0.22954444444444444444e0 * t19102;
    let t21823 = t5788 * t3725;
    let t21869 = t2285 * t4435;
    let t21872 = t2097 * t3696;
    let t21900 = t6453 * t1576;
    let t21902 = t2318 * t4416;
    let t21908 = 0.17990788716177317213e-1 * t6450 * t1576;
    let t21937 = t20160 * t6506;
    let t21939 = 0.35981577432354634426e-1 * t1580 * t21937;
    (t21771, t21804, t21823, t21869, t21872, t21900, t21902, t21908, t21939)
}
