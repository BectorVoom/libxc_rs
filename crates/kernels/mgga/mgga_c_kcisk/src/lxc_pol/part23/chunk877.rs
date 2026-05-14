//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 877/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk877<F: Float>(t1216: F, t3929: F, t1447: F, t3805: F, t3783: F, t394: F, t1327: F, t3924: F, t1458: F, t4163: F, t1455: F, t4169: F, sigma0: F) -> (F, F, F, F, F, F, F) {
    let t14242 = t1216 * t3929;
    let t14250 = t3805 * t1447;
    let t14264 = t3783 * sigma0;
    let t14265 = t14264 * t394;
    let t14273 = t1327 * t3924;
    let t14284 = t4163 * t1458;
    let t14287 = t1455 * t4169;
    (t14242, t14250, t14264, t14265, t14273, t14284, t14287)
}
