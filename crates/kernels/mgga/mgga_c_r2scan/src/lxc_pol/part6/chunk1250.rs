//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1250/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1250<F: Float>(t1248: F, t803: F, t295: F, t6621: F, t806: F, t990: F, t305: F, t6635: F, t1000: F, t810: F, t23399: F, t1217: F, t1218: F, t1249: F, t1257: F, t1261: F, t23384: F, t23394: F, t23407: F, t4911: F, t6641: F, t8319: F, t8320: F, t8323: F, t8340: F, t997: F) -> (F,) {
    let t23459 = t803 * t1248;
    let t23462 = t295 * t6621;
    let t23463 = t990 * t806;
    let t23473 = t305 * t6635;
    let t23474 = t1000 * t810;
    let t23491 = 20.0 * t305 * t23399;
    let t23492 = -50.0 / 9.0 * t803 * t8323 + 10.0 / 9.0 * t295 * t23407 - 50.0 / 9.0 * t997 * t6641 + 40.0 / 81.0 * t305 * t23384 - 10.0 / 3.0 * t305 * t23394 - 100.0 / 9.0 * t23459 * t8320 - 10.0 / 9.0 * t23462 * t23463 * t1218 - 10.0 / 9.0 * t23462 * t1217 * t1249 + 10.0 / 3.0 * t8319 * t1217 * t1218 - 10.0 / 9.0 * t23473 * t23474 * t1261 + 10.0 / 9.0 * t23473 * t1217 * t1257 - 10.0 / 3.0 * t8340 * t1217 * t1261 - 10.0 * t8319 * t4911 * t806 + 10.0 * t8340 * t4911 * t810 - t23491;
    (t23492,)
}
