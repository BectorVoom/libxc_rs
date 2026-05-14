//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1059/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1059<F: Float>(t21139: F, t21181: F, t21221: F, t21266: F, t467: F, t488: F, t4223: F, t6336: F, t14581: F, t6313: F, t21058: F, t21060: F, t21062: F, t21064: F, t21067: F, t21069: F, t21072: F, t21075: F, t21078: F, t21081: F, t21083: F, t21086: F, t21088: F, t21091: F, t21094: F, t21096: F, t21099: F, sigma0: F) -> (F, F, F, F, F) {
    let t21268 = t21139 + t21181 + t21221 + t21266;
    let t21269 = t21268 * t467;
    let t21270 = t21269 * sigma0;
    let t21271 = t21270 * t488;
    let t21273 = t4223 * t6336;
    let t21275 = t14581 * t6313;
    let t21277 = t21058 / 3.0 + t21060 / 96.0 + t21062 / 24.0 + t21064 / 96.0 - t21067 / 128.0 + t21069 / 3.0 - t21072 / 96.0 + t21075 / 192.0 + t21078 / 8.0 + 2.0 / 27.0 * t21081 - t21083 / 192.0 + t21086 / 192.0 + t21088 / 18.0 + t21091 / 72.0 + t21094 / 288.0 - t21096 / 3.0 - t21099 / 64.0 + t21271 / 16.0 - t21273 / 24.0 - t21275 / 12.0;
    (t21269, t21271, t21273, t21275, t21277)
}
