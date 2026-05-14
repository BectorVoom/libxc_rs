//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 979/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk979<F: Float>(t3739: F, t5887: F, t2211: F, t3783: F, t3787: F, t1411: F, t3791: F, t5886: F, t1333: F, t5869: F, t3512: F, t5602: F, t5600: F, t6343: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t19846 = t3739 * t5887;
    let t19847 = 0.33163888888888888888e-2 * t19846;
    let t19848 = t2211 * t3783;
    let t19849 = t19848 * sigma0;
    let t19850 = t19849 * t3787;
    let t19851 = t1411 * t19850;
    let t19853 = t5886 * t3791;
    let t19854 = t1411 * t19853;
    let t19856 = t1333 * t5869;
    let t19857 = 0.33163888888888888888e-2 * t19856;
    let t19858 = t3512 * t5602;
    let t19859 = t5600 * t19858;
    let t19861 = t6343 * sigma0;
    (t19846, t19847, t19848, t19849, t19851, t19854, t19856, t19857, t19859, t19861)
}
