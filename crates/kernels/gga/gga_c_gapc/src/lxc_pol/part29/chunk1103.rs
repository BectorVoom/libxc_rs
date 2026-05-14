//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1103/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1103<F: Float>(t10350: F, t11678: F, t35734: F, t11679: F, t24202: F, t11656: F, t11658: F, t24181: F, t11214: F, t11663: F, t6853: F, t760: F, t10343: F, t3734: F, t10336: F, t291: F, t640: F) -> (F, F, F, F, F, F, F) {
    let t35736 = t35734 * t11678 * t10350;
    let t35738 = t11679 * t24202;
    let t35741 = t24181 * t11656 * t11658;
    let t35745 = t11214 * t760 * t6853 * t11663;
    let t35747 = t10343 * t3734;
    let t35749 = t10336 * t3734;
    let t35751 = t640 * t291;
    (t35736, t35738, t35741, t35745, t35747, t35749, t35751)
}
