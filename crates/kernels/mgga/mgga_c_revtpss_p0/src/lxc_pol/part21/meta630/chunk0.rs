//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2396/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2396<F: Float>(t40769: F, t810: F, t820: F, t849: F, t9948: F, t857: F, t10722: F, t2479: F, t14832: F, t2430: F, t2475: F, t2661: F, t775: F) -> (F, F, F, F, F) {
    let t40771 = F::cast_from(0.70398079132139197745e-2_f64) * t40769 * t810;
    let t40781 = t820 * t849 * t9948;
    let t40782 = t40781 * t857;
    let t40784 = t10722 * t2479;
    let t40789 = t2661 * t14832 * t2475 * t775 * t2430;
    (t40771, t40781, t40782, t40784, t40789)
}
