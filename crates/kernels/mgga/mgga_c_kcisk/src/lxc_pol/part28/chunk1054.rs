//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1054/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1054<F: Float>(t24155: F, t5321: F, t17936: F, t7307: F, t22360: F, t7311: F, t22278: F, t5290: F, t7315: F, t7320: F, t7437: F, t11717: F, t9072: F, t1945: F, t9066: F, t17976: F, t7440: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t24156 = t5321 * t24155;
    let t24158 = t17936 * t7307;
    let t24160 = t7311 * t22360;
    let t24161 = t5321 * t24160;
    let t24163 = t5290 * t22278;
    let t24164 = t7315 * t24163;
    let t24166 = t7320 * t7437;
    let t24168 = t11717 * t9072;
    let t24170 = t1945 * t9066;
    let t24172 = t17976 * t7440;
    (t24156, t24158, t24160, t24161, t24163, t24164, t24166, t24168, t24170, t24172)
}
