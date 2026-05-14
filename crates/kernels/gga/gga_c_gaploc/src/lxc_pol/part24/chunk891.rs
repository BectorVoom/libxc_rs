//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 891/1270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk891<F: Float>(t10451: F, t1445: F, t3354: F, t4673: F, t1572: F, t3384: F, t4950: F, t10414: F, t10415: F, t10416: F, t10418: F, t10423: F, t10426: F, t10428: F, t10433: F, t10437: F, t10441: F, t10443: F, t10446: F, t10450: F, t1424: F, t1450: F) -> (F, F, F) {
    let t10452 = t1445 * t10451;
    let t10455 = t4673 * t3354;
    let t10457 = 0.47667319935800568892e0 * t1572 * t10455;
    let t10459 = 0.71500979903700853338e0 * t4950 * t3384;
    let t10460 = t10414 - t10415 + t10416 - 0.39722766613167140743e-1 * t10418 * t1424 - t10423 + t10426 + t10428 + t10433 - t10437 + t10441 - t10443 - t10446 - t10450 - 0.23005755572352449806e1 * t1450 * t10452 + t10457 + t10459;
    (t10452, t10455, t10460)
}
