//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 487/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk487<F: Float>(t3805: F, t472: F, t1333: F, t1447: F, t1407: F, t300: F, t967: F, t425: F, t1350: F, t443: F, t1346: F, t1365: F) -> (F, F, F, F, F, F, F, F) {
    let t3806 = t3805 * t472;
    let t3807 = F::cast_from(0.55273148148148148147e-3_f64) * t3806;
    let t3808 = t1333 * t1447;
    let t3810 = t1333 * t1407;
    let t3812 = t967 * t300;
    let t3814 = F::cast_from(0.46853067927761790996e-2_f64) * t3812 * t425;
    let t3815 = t443 * t1350;
    let t3817 = t1346 * t1365;
    (t3806, t3807, t3808, t3810, t3812, t3814, t3815, t3817)
}
