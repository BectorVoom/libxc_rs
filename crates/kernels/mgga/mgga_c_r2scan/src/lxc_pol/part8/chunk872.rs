//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 872/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk872<F: Float>(t2593: F, t7494: F, t1610: F, t2207: F, t2691: F, t2530: F, t537: F, t6217: F, t7460: F) -> (F, F, F, F) {
    let t7496 = 0.25610080155860322884e0 * t7494 * t2593;
    let t7500 = 0.34930954652346593434e-1 * t2207 * t1610 * t2691;
    let t7503 = t537 * t2530;
    let t7512 = t6217 * t7460;
    (t7496, t7500, t7503, t7512)
}
