//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1201/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1201<F: Float>(t5572: F, t9541: F, t5624: F, t9601: F, t1512: F, t47092: F, t16673: F, t2642: F, t5614: F, t9671: F, t41008: F, t5568: F) -> (F, F, F, F, F, F) {
    let t58550 = t9541 * t5572;
    let t58574 = t9601 * t5624;
    let t58576 = t47092 * t1512;
    let t58642 = t16673 * t2642;
    let t58723 = t9671 * t5614;
    let t58744 = t41008 * t5568;
    (t58550, t58574, t58576, t58642, t58723, t58744)
}
