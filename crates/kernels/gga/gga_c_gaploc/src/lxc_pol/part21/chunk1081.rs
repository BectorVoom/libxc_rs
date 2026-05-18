//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1081/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1081<F: Float>(t25729: F, t6508: F, t2754: F, t447: F, t2366: F, t1305: F, t986: F, t197: F, t161: F, t4538: F, t599: F, t7861: F) -> (F, F, F, F, F, F, F) {
    let t25730 = t6508 * t25729;
    let t25734 = t2754 * t447;
    let t25735 = t2366 * t25734;
    let t25739 = t986 * t1305;
    let t25740 = t2366 * t25739;
    let t25760 = t197 * t2754;
    let t25761 = t25760 * t161;
    let t25775 = t4538 * t986;
    let t25841 = t599 * t7861;
    (t25730, t25735, t25740, t25760, t25761, t25775, t25841)
}
