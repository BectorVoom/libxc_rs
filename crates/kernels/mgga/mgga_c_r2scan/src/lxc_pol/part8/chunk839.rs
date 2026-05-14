//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 839/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk839<F: Float>(t2158: F, t6407: F, t1415: F, t511: F, t2162: F, t2164: F, t1607: F, t5100: F, t512: F, t6101: F, t507: F, t1591: F, t2168: F, t1541: F, t545: F) -> (F, F, F, F, F, F, F, F) {
    let t6408 = t6407 * t2158;
    let t6412 = t1415 * t511;
    let t6415 = 0.89443204944342177673e-3 * t6412 * t2162 * t2164;
    let t6420 = t5100 * t1607;
    let t6422 = t512 * t6101;
    let t6424 = 0.174549769648958674e0 * t6422 * t507;
    let t6425 = t1591 * t2168;
    let t6448 = t545 * t1541;
    (t6408, t6412, t6415, t6420, t6422, t6424, t6425, t6448)
}
