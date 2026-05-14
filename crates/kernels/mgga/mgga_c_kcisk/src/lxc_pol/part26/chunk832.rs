//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 832/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk832<F: Float>(t14223: F, t1442: F, t1452: F, t1216: F, t3929: F, t1447: F, t3805: F, t3532: F, t382: F, t3783: F, t394: F, t1455: F, t4169: F, t1457: F, t475: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t14224 = t14223 * t1442;
    let t14226 = t14223 * t1452;
    let t14242 = t1216 * t3929;
    let t14250 = t3805 * t1447;
    let t14255 = t382 * t3532;
    let t14264 = t3783 * sigma0;
    let t14265 = t14264 * t394;
    let t14287 = t1455 * t4169;
    let t14292 = t1457 * t1457;
    let t14293 = 1.0 / t14292;
    let t14294 = t475 * t14293;
    (t14224, t14226, t14242, t14250, t14255, t14264, t14265, t14287, t14292, t14293, t14294)
}
