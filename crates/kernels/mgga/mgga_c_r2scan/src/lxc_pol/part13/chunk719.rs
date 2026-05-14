//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 719/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk719<F: Float>(t1607: F, t5100: F, t512: F, t6101: F, t507: F, t1591: F, t2168: F, t1584: F, t1634: F, t1551: F, t1632: F, t551: F, t574: F, t1541: F, t545: F, t548: F) -> (F, F, F, F, F, F, F) {
    let t6420 = t5100 * t1607;
    let t6422 = t512 * t6101;
    let t6424 = 0.174549769648958674e0 * t6422 * t507;
    let t6425 = t1591 * t2168;
    let t6440 = t1584 * t1634;
    let t6445 = t551 * t1632 * t1551;
    let t6446 = t574 * t6445;
    let t6448 = t545 * t1541;
    let t6449 = t6448 * t548;
    (t6420, t6424, t6425, t6440, t6446, t6448, t6449)
}
