//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 812/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk812<F: Float>(t225: F, t5429: F, t1719: F, t1986: F, t5317: F, t721: F, t1647: F, t645: F, t650: F, t648: F, t14: F, t651: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5658 = t5429 * t225;
    let t5661 = t1986 * t1719;
    let t5664 = t721 * t5317;
    let t5669 = 18.0 * t650 * t645 * t1647;
    let t5670 = t648 * t648;
    let t5671 = 1.0 / t5670;
    let t5672 = t14 * t5671;
    let t5673 = t651 * t651;
    let t5674 = 1.0 / t5673;
    (t5658, t5661, t5664, t5669, t5670, t5671, t5672, t5673, t5674)
}
