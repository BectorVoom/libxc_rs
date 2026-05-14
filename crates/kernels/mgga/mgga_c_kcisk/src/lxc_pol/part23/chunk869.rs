//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 869/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk869<F: Float>(t1314: F, t13900: F, t1309: F, t3966: F, t3974: F, t3970: F, t3947: F, t3973: F, t25: F, t3951: F) -> (F, F, F, F, F) {
    let t13901 = t13900 * t1314;
    let t13902 = t1309 * t13901;
    let t13906 = t3966 * t3974;
    let t13910 = t3970 * t3974;
    let t13912 = t3973 * t3947;
    let t13913 = t1309 * t13912;
    let t13917 = t25 * t3951;
    (t13902, t13906, t13910, t13913, t13917)
}
