//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 503/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk503<F: Float>(t1771: F, t637: F, t160: F, t36: F, t164: F, t601: F, t604: F, t1414: F, t162: F, t161: F) -> (F, F, F, F, F, F, F) {
    let t1772 = t1771 * t637;
    let t1774 = t160 * t36;
    let t1776 = 132.0 * t1774 * t164;
    let t1778 = 288.0 * t601 * t604;
    let t1779 = t162 * t1414;
    let t1780 = 1.0 / t1779;
    let t1782 = 156.0 * t161 * t1780;
    let t1783 = -t1776 + t1778 - t1782;
    (t1772, t1774, t1776, t1779, t1780, t1782, t1783)
}
