//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 771/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk771<F: Float>(t1620: F, t2682: F, t129: F, t1598: F, t524: F, t2593: F, t1610: F, t2207: F, t2691: F, t2530: F, t537: F, t2124: F, t2551: F, t495: F, t6217: F, t7460: F) -> (F, F, F, F, F, F, F, F) {
    let t7490 = t1620 * t2682;
    let t7494 = t524 * t1598 * t129;
    let t7496 = 0.25610080155860322884e0 * t7494 * t2593;
    let t7500 = 0.34930954652346593434e-1 * t2207 * t1610 * t2691;
    let t7503 = t537 * t2530;
    let t7505 = t2124 * t7503 * t2551;
    let t7509 = t2124 * t7503 * t495;
    let t7512 = t6217 * t7460;
    (t7490, t7494, t7496, t7500, t7503, t7505, t7509, t7512)
}
