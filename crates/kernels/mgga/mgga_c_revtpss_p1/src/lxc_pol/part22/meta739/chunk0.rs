//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2801/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2801<F: Float>(t40769: F, t810: F, t820: F, t849: F, t9948: F, t857: F, t10722: F, t2479: F, t2699: F, t2729: F, t2732: F, t235: F, t4503: F) -> (F, F, F, F, F, F, F) {
    let t40771 = F::cast_from(0.70398079132139197745e-2_f64) * t40769 * t810;
    let t40781 = t820 * t849 * t9948;
    let t40782 = t40781 * t857;
    let t40784 = t10722 * t2479;
    let t40791 = t2699 * t2729;
    let t40792 = t40791 * t2732;
    let t40798 = t4503 * t235;
    (t40771, t40781, t40782, t40784, t40791, t40792, t40798)
}
