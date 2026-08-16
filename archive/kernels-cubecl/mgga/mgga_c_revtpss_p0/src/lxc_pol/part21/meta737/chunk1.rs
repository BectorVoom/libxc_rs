//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2589/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2589<F: Float>(t2435: F, t9667: F, t268: F, t39644: F, t556: F, t561: F, t8779: F, t786: F, t9656: F, t686: F, t72: F, t9658: F) -> (F, F, F, F) {
    let t47595 = t2435 * t9667;
    let t47601 = F::cast_from(0.11638313500518478545e-4_f64) * t39644 * t556 * t561 * t8779 * t268;
    let t47603 = t786 * t556 * t9656;
    let t47606 = t47603 * t9658 * t72 * t686;
    (t47595, t47601, t47603, t47606)
}
