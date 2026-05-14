//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1229/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1229<F: Float>(t22767: F, t6195: F, t6205: F, t6231: F, t1592: F, t1632: F, t551: F, t6435: F, t20137: F, t6209: F, t6213: F, t546: F, t8021: F, t565: F, t113: F, t20084: F) -> (F, F, F, F, F, F, F) {
    let t22768 = t22767 * t6195;
    let t22770 = t6205 * t6231;
    let t22775 = t1592 * t551 * t1632 * t6435;
    let t22778 = t6209 * t20137 * t6213;
    let t22780 = t546 * t8021;
    let t22783 = t565 * t8021;
    let t22786 = t20084 * t113;
    (t22768, t22770, t22775, t22778, t22780, t22783, t22786)
}
