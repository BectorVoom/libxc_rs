//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1253/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1253<F: Float>(t322: F, t23519: F, t1013: F, t1292: F, t1295: F, t1299: F, t1300: F, t19203: F, t2394: F, t2397: F, t2400: F, t327: F, t6682: F, t6688: F, t6692: F, t6693: F, t6696: F, t829: F, t833: F, t834: F, t8398: F, t8401: F, t8404: F) -> (F,) {
    let t324 = 0.0 < t322;
    let t23520 = piecewise3(t324, 0.0, t23519);
    let t23538 = -0.1152e2 * t8404 * t6696 - 0.1152e2 * t6693 * t2394 * t1295 - 0.1536e2 * t19203 * t1013 * t6688 - 0.384e1 * t1300 * t8398 * t829 - 0.384e1 * t1300 * t2394 * t1292 - 0.128e1 * t1300 * t1013 * t6682 - 0.1152e2 * t6693 * t2400 * t1292 - 0.64e0 * t23520 * t327 - 0.384e1 * t8398 * t833 * t829 - 0.384e1 * t8401 * t1292 - 0.1152e2 * t2394 * t1299 * t1295 - 0.1536e2 * t1013 * t6692 * t6688 - 0.128e1 * t2397 * t6682 - 0.64e0 * t834 * t23520;
    (t23538,)
}
