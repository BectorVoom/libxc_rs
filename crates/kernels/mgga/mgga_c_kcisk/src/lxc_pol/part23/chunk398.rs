//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 398/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk398<F: Float>(t1375: F, t2075: F, t1383: F, t1398: F, t1349: F, t1369: F, t1374: F, t1382: F, t1388: F, t1391: F, t1397: F, t158: F, t165: F, t173: F, t2059: F) -> (F, F, F, F) {
    let t2198 = t1375 * t2075;
    let t2201 = t1383 * t2075;
    let t2206 = t1398 * t2075;
    let t2209 = t1369 + 0.11955719325063177623e-1 * t1349 * t2059 - t1374 - 0.3513e-2 * t158 * t2198 + t1382 + 0.7925e-3 * t165 * t2201 - t1388 - 0.5179538907796306876e-4 * t1391 * t2059 + t1397 + 0.50413125e-5 * t173 * t2206;
    (t2198, t2201, t2206, t2209)
}
