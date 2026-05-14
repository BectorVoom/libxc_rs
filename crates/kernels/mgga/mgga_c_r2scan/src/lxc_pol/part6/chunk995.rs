//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 995/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk995<F: Float>(t506: F, t529: F, t7591: F, t551: F, t6343: F, t921: F, t574: F, t2145: F, t978: F, t146: F, t2151: F, t494: F, t910: F, t113: F, t538: F, t6155: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t7593 = t529 * t506 * t7591;
    let t7597 = t551 * t6343 * t921;
    let t7598 = t574 * t7597;
    let t7600 = t2145 * t978;
    let t7601 = t146 * t7600;
    let t7603 = 0.11643651550782197811e-1 * t7601 * t2151;
    let t7604 = t910 * t494;
    let t7605 = t7604 * t113;
    let t7606 = t538 * t7605;
    let t7608 = 0.10975748638225852664e-1 * t6155 * t7606;
    (t7593, t7597, t7598, t7600, t7601, t7603, t7604, t7605, t7606, t7608)
}
