//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1213/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1213<F: Float>(t120: F, t9660: F, t10003: F, t9638: F, t10009: F, t2617: F, t9600: F, t849: F, t2707: F, t9993: F, t2642: F, t9612: F) -> (F, F, F, F, F, F) {
    let t41039 = t120 * t9660;
    let t41048 = t9638 * t10003;
    let t41050 = t9638 * t10009;
    let t41052 = t2617 * t9600;
    let t41053 = t41052 * t849;
    let t41055 = t9993 * t2707;
    let t41063 = t9612 * t2642;
    (t41039, t41048, t41050, t41053, t41055, t41063)
}
