//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1355/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1355<F: Float>(t113: F, t1550: F, t910: F, t538: F, t6155: F, t494: F, t9507: F, t1568: F, t7623: F, t1610: F, t2201: F, t7476: F, t1553: F, t24209: F, t6375: F, t24172: F) -> (F, F, F, F, F, F, F, F) {
    let t25670 = t910 * t1550 * t113;
    let t25672 = t6155 * t538 * t25670;
    let t25684 = t9507 * t494;
    let t25686 = t7623 * t1568 * t25684;
    let t25695 = t2201 * t1610 * t7476;
    let t25697 = t24209 * t1553;
    let t25699 = t7623 * t6375 * t25697;
    let t25702 = t7623 * t538 * t24172;
    (t25670, t25672, t25684, t25686, t25695, t25697, t25699, t25702)
}
