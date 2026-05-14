//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 858/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk858<F: Float>(t1195: F, t6727: F, t382: F, t3477: F, t6724: F, t14721: F, t1813: F, t1805: F, t5165: F, t15068: F, t5062: F, t10796: F, t6717: F, t3474: F, t6697: F, t19630: F, t3338: F) -> (F, F, F, F, F, F, F, F, F) {
    let t19911 = t1195 * t6727;
    let t19912 = t382 * t19911;
    let t19914 = t3477 * t6724;
    let t19916 = t14721 * t1813;
    let t19918 = t5165 * t1805;
    let t19920 = t15068 * t5062;
    let t19922 = t10796 * t6717;
    let t19924 = t3474 * t6697;
    let t19926 = t3338 * t19630;
    (t19911, t19912, t19914, t19916, t19918, t19920, t19922, t19924, t19926)
}
