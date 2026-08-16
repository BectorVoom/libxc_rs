//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2738/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2738(t1799: f64, t3698: f64, t20063: f64, t3701: f64, t1388: f64, t15899: f64, t3918: f64, t39642: f64, t39655: f64, t39658: f64, t5160: f64, t57206: f64, t57207: f64, t57209: f64, t57210: f64, t57212: f64, t57213: f64, t57214: f64) -> f64 {
    let t57802 = t1799 * t3698;
    let t57806 = t20063 * t3701;
    let t57810 = -2.0_f64 * t1388 * t5160 * t57806 + 12.0_f64 * t15899 * t3918 * t57802 + t39642 - t39655 - t39658 + t57206 + t57207 + t57209 + t57210 + t57212 - t57213 + t57214;
    t57810
}
