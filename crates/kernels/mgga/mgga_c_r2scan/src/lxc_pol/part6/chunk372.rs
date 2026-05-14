//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 372/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk372<F: Float>(t1267: F, t1243: F, t1250: F, t1253: F, t1258: F, t1262: F, t295: F, t299: F, t305: F, t803: F, t807: F, t815: F, t818: F) -> (F, F, F) {
    let t1268 = 11.0 / 9.0 * t1267;
    let t1269 = 40.0 / 9.0 * t1243 * t299 - 50.0 / 9.0 * t803 * t807 + 10.0 / 9.0 * t295 * t1250 + 5.0 / 3.0 * t295 * t1253 + 10.0 / 9.0 * t305 * t1258 + 5.0 / 3.0 * t305 * t1262 - t1268;
    let t1271 = t815 * t818;
    (t1268, t1269, t1271)
}
