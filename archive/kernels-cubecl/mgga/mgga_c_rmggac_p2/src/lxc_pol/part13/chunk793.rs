//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 793/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk793<F: Float>(t36541: F, t7473: F, t1969: F, t34846: F, t2001: F, t2002: F, t305: F, t321: F, t7345: F, t7927: F, t35207: F, t7354: F) -> (F, F, F, F, F) {
    let t36769 = t36541 * t7473;
    let t36772 = t34846 * t1969;
    let t36787 = t2001 * t305 * t2002 * t321;
    let t36796 = t7345 * t7927;
    let t36801 = t35207 * t7354;
    (t36769, t36772, t36787, t36796, t36801)
}
