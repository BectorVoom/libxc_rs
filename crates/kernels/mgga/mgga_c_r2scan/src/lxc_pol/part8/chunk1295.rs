//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1295/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1295<F: Float>(t133: F, t1604: F, t1605: F, t8629: F, t1632: F, t551: F, t6449: F, t9182: F, t6425: F, t9232: F, t22796: F, t9297: F, t25983: F, t8161: F, t146: F, t5094: F, t978: F) -> (F, F, F, F, F, F) {
    let t30772 = t1604 * t1605 * t133 * t8629;
    let t30777 = t6449 * t551 * t1632 * t9182;
    let t30779 = t6425 * t9232;
    let t30787 = t22796 * t9297;
    let t30789 = t25983 * t8161;
    let t30792 = t146 * t5094 * t978;
    (t30772, t30777, t30779, t30787, t30789, t30792)
}
