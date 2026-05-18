//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 973/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk973<F: Float>(t5055: F, t7444: F, t236: F, t321: F, t3351: F, t35312: F, t9211: F, t2329: F, t36669: F, t1970: F, t1971: F, t209: F, t40444: F, t511: F) -> (F, F, F, F) {
    let t40630 = t5055 * t7444;
    let t40637 = t3351 * t35312 * t236 * t9211 * t321;
    let t40647 = t36669 * t2329;
    let t40652 = t1970 * t1971 * t511 * t40444 * t209;
    (t40630, t40637, t40647, t40652)
}
