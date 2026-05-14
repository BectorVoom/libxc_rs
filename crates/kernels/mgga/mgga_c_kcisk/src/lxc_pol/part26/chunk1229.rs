//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1229/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1229<F: Float>(t12671: F, t31848: F, t12769: F, t2689: F, t982: F, t12705: F, t9352: F, t12698: F, t12699: F, t3132: F, t31854: F, t116: F, t12664: F, t3138: F, t3174: F, t9355: F) -> (F, F, F, F, F, F, F) {
    let t110824 = t12671 * t31848;
    let t110827 = t982 * t2689 * t12769;
    let t110829 = t12705 * t9352;
    let t110832 = t12698 * t2689 * t12699;
    let t110834 = t3132 * t31854;
    let t110837 = t3138 * t116 * t12664;
    let t110840 = t982 * t9355 * t3174;
    (t110824, t110827, t110829, t110832, t110834, t110837, t110840)
}
