//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1277/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1277<F: Float>(t32019: F, t32176: F, t32065: F, t3969: F, t20160: F, t32034: F, t9446: F, t32029: F, t9426: F, t32018: F, t1308: F, t388: F, t39814: F, t1292: F, t14242: F, t13820: F, t9425: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t110279 = t32019 * t32176;
    let t110281 = t32065 * t3969;
    let t110284 = t20160 * t32034;
    let t110285 = t9446 * t110284;
    let t110289 = t20160 * t32029;
    let t110290 = t9426 * t110289;
    let t110294 = t32018 * t3969;
    let t110304 = t39814 * t388 * t1308;
    let t110308 = t14242 * t1292 * t1308;
    let t110319 = t9425 * t13820;
    (t110279, t110281, t110284, t110285, t110289, t110290, t110294, t110304, t110308, t110319)
}
