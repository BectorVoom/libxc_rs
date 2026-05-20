//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2913/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2913<F: Float>(t11606: F, t4719: F, t1642: F, t41491: F, t11591: F, t4729: F, t52229: F, t52231: F, t52235: F, t52237: F, t52242: F, t52245: F, t52860: F, t52863: F) -> (F, F, F, F) {
    let t52865 = F::cast_from(0.10389515463408878255e3_f64) * t4719 * t11606;
    let t52867 = F::cast_from(0.5848223622634646207e0_f64) * t41491 * t1642;
    let t52869 = F::cast_from(0.17544670867903938621e1_f64) * t11591 * t4729;
    let t52870 = t52229 + t52231 + t52235 + t52237 + t52242 + t52245 + t52860 + t52863 + t52865 - t52867 - t52869;
    (t52865, t52867, t52869, t52870)
}
