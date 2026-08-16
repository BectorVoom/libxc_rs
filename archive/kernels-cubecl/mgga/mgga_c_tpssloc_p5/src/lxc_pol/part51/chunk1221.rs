//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1221/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1221<F: Float>(t1985: F, t33310: F, t1842: F, t8636: F, t3887: F, t2091: F, t7749: F, t26989: F, t7728: F, t1375: F, t26224: F, t31649: F, t31663: F, t33308: F, t5215: F, t5321: F, t6958: F, t7194: F, t7729: F, t7925: F, t8627: F) -> (F, F, F, F) {
    let t33311 = t1985 * t33310;
    let t33315 = t8636 * t1842;
    let t33316 = t3887 * t33315;
    let t33320 = t3887 * t2091 * t7749;
    let t33323 = t26989 * t7728;
    let t33332 = -F::cast_from(0.16449340668482264365e-1_f64) * t33308 - F::cast_from(0.82246703342411321825e-2_f64) * t33311 + t31649 + F::cast_from(2.0_f64) * t5215 * t8627 + F::cast_from(2.0_f64) * t1375 * t33316 + F::cast_from(2.0_f64) * t1375 * t33320 - F::cast_from(6.0_f64) * t26224 * t33323 + F::cast_from(2.0_f64) * t5321 * t8627 + F::cast_from(2.0_f64) * t7194 * t7729 - t31663 + F::cast_from(2.0_f64) * t6958 * t7925;
    (t33316, t33320, t33323, t33332)
}
