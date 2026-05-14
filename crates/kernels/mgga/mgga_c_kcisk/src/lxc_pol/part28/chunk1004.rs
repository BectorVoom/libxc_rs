//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1004/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1004<F: Float>(t23176: F, t23208: F, t23233: F, t23269: F, t11460: F, t16088: F, t16090: F, t16105: F, t16107: F, t16122: F, t16124: F, t16298: F, t16303: F, t1689: F, t1809: F, t1860: F, t22440: F, t22514: F, t22919: F, t23108: F, t23111: F, t23115: F, t23118: F, t2399: F, t2505: F, t604: F, t674: F, t6884: F, t6941: F, t702: F, t8616: F, t8662: F) -> (F, F) {
    let t23271 = t23176 + t23208 + t23233 + t23269;
    let t23274 = 0.28111840756657074598e-1 * t674 * t23108 + 0.46853067927761790996e-2 * t1809 * t23111 + 0.14055920378328537299e-1 * t674 * t23115 - 0.93706135855523581992e-2 * t23118 - 0.18741227171104716398e-1 * t16303 * t22514 - 0.28111840756657074598e-1 * t16298 * t22440 - 0.93706135855523581992e-2 * t16088 - 0.18741227171104716398e-1 * t16090 - t11460 - t16105 - t16107 - t16122 - t16124 - 2.0 * t6884 * t2505 - t8616 * t1860 - t1689 * t8662 - 2.0 * t2399 * t6941 - t604 * t23271 - t22919 * t702;
    (t23271, t23274)
}
