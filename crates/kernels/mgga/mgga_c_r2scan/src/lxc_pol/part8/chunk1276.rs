//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1276/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1276<F: Float>(t2155: F, t29775: F, t6063: F, t2526: F, t2567: F, t2148: F, t7614: F, t3056: F, t481: F, t538: F, t6155: F, t20762: F, t29467: F, t1592: F, t1632: F, t551: F, t9207: F) -> (F, F, F, F, F, F) {
    let t29777 = t2155 * t6063 * t29775;
    let t29779 = t2567 * t2526;
    let t29781 = t7614 * t2148 * t29779;
    let t29783 = t3056 * t481;
    let t29785 = t6155 * t538 * t29783;
    let t29788 = t20762 * t538 * t29467;
    let t29798 = t1592 * t551 * t1632 * t9207;
    (t29777, t29781, t29783, t29785, t29788, t29798)
}
