//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 882/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk882<F: Float>(t1504: F, t68: F, t1891: F, t5527: F, t1509: F, t1519: F, t5550: F, t9573: F, t213: F, t118: F, t794: F, t9549: F) -> (F, F, F, F, F, F, F) {
    let t16729 = t1504 * t68;
    let t16736 = t1891 * t5527;
    let t16758 = t1519 * t1509;
    let t16769 = t9573 * t5550;
    let t16771 = t213 * t5527;
    let t16783 = t118 * t794 * t5527;
    let t16784 = t9549 * t16783;
    (t16729, t16736, t16758, t16769, t16771, t16783, t16784)
}
