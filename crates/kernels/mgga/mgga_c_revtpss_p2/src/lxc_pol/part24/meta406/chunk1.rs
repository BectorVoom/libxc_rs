//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1346/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1346<F: Float>(t40757: F, t810: F, t240: F, t9731: F, t10293: F, t124: F, t212: F, t800: F, t820: F, t849: F, t9948: F, t2699: F, t2729: F) -> (F, F, F, F, F, F) {
    let t40759 = F::cast_from(0.26776076960158126592e-7_f64) * t40757 * t810;
    let t40763 = t9731 * t240;
    let t40769 = t800 * t124 * t10293 * t212;
    let t40771 = F::cast_from(0.70398079132139197745e-2_f64) * t40769 * t810;
    let t40781 = t820 * t849 * t9948;
    let t40791 = t2699 * t2729;
    (t40759, t40763, t40769, t40771, t40781, t40791)
}
