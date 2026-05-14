//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 821/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk821<F: Float>(t3923: F, t408: F, t3936: F, t3959: F, t1299: F, t389: F, t3934: F, t1319: F, t4065: F) -> (F, F, F, F) {
    let t13440 = 1.0 / t3923 / t408;
    let t13472 = t3936 * t3959;
    let t13482 = t389 * t1299 * t3934;
    let t13485 = t4065 * t1319;
    (t13440, t13472, t13482, t13485)
}
