//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 882/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk882(t1985: f64, t33310: f64, t1842: f64, t8636: f64, t3887: f64, t2091: f64, t7749: f64, t26989: f64, t7728: f64, t1375: f64, t26224: f64, t31649: f64, t31663: f64, t33308: f64, t5215: f64, t5321: f64, t6958: f64, t7194: f64, t7729: f64, t7925: f64, t8627: f64) -> (f64, f64, f64, f64) {
    let t33311 = t1985 * t33310;
    let t33315 = t8636 * t1842;
    let t33316 = t3887 * t33315;
    let t33320 = t3887 * t2091 * t7749;
    let t33323 = t26989 * t7728;
    let t33332 = -0.16449340668482264365e-1_f64 * t33308 - 0.82246703342411321825e-2_f64 * t33311 + t31649 + 2.0_f64 * t5215 * t8627 + 2.0_f64 * t1375 * t33316 + 2.0_f64 * t1375 * t33320 - 6.0_f64 * t26224 * t33323 + 2.0_f64 * t5321 * t8627 + 2.0_f64 * t7194 * t7729 - t31663 + 2.0_f64 * t6958 * t7925;
    (t33316, t33320, t33323, t33332)
}
