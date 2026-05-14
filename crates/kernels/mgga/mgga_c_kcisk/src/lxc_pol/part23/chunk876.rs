//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 876/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk876<F: Float>(t394: F, t4143: F, t1284: F, t1299: F, t3504: F, t3739: F, t3509: F, t10471: F, t140: F, t416: F, t1442: F, t1452: F, t3496: F, t3744: F, t3748: F, t3766: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14208 = t394 * t4143;
    let t14213 = t1299 * t1284;
    let t14218 = t3739 * t3504;
    let t14220 = t3739 * t3509;
    let t14223 = t140 * t10471 * t416;
    let t14224 = t14223 * t1442;
    let t14226 = t14223 * t1452;
    let t14228 = t3739 * t3496;
    let t14230 = t3739 * t3744;
    let t14232 = t3748 * t3766;
    (t14208, t14213, t14218, t14220, t14223, t14224, t14226, t14228, t14230, t14232)
}
