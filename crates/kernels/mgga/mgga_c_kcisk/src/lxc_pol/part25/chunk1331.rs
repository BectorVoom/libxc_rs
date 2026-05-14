//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1331/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1331<F: Float>(t17775: F, t33083: F, t33068: F, t7444: F, t17979: F, t9708: F, t34329: F, t5291: F, t17891: F, t34368: F, t17799: F, t5062: F, t112051: F, t7317: F, t33091: F, t7299: F) -> (F, F, F, F, F, F, F, F) {
    let t117265 = 4.0 * t17775 * t33083;
    let t117267 = 2.0 * t33068 * t7444;
    let t117268 = t9708 * t17979;
    let t117270 = t34329 * t5291;
    let t117272 = t34368 * t17891;
    let t117274 = t5062 * t17799;
    let t117276 = t112051 * t7317;
    let t117278 = t33091 * t7299;
    (t117265, t117267, t117268, t117270, t117272, t117274, t117276, t117278)
}
