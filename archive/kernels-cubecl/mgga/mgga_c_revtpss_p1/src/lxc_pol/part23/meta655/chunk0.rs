//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2383/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2383<F: Float>(t40769: F, t810: F, t820: F, t849: F, t9948: F, t857: F, t2699: F, t2729: F, t235: F, t4503: F, t2453: F, t123: F, t125: F, t2452: F, t40633: F) -> (F, F, F, F, F, F, F) {
    let t40771 = F::cast_from(0.70398079132139197745e-2_f64) * t40769 * t810;
    let t40781 = t820 * t849 * t9948;
    let t40782 = t40781 * t857;
    let t40791 = t2699 * t2729;
    let t40798 = t4503 * t235;
    let t40799 = t2453 * t40798;
    let t40810 = F::cast_from(0.30119321664969771194e-5_f64) * t123 * t125 * t40633 * t2452 * t810;
    (t40771, t40781, t40782, t40791, t40798, t40799, t40810)
}
