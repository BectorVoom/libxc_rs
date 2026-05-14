//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 689/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk689<F: Float>(t6147: F, t395: F, t1300: F, t2159: F, t1305: F, t2160: F, t1308: F, sigma0: F) -> (F, F, F, F, F) {
    let t6148 = t6147 * sigma0;
    let t6149 = t6148 * t395;
    let t6152 = t2159 * t1300;
    let t6155 = t2160 * t1305;
    let t6157 = t2159 * t1308;
    (t6148, t6149, t6152, t6155, t6157)
}
