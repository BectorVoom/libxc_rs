//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1302/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1302<F: Float>(t1307: F, t221: F, t5196: F, t118: F, t1799: F, t794: F, t3739: F, t210: F, t214: F, t5187: F, t1315: F, t3725: F, t3727: F, t3731: F, t3742: F, t3751: F, t5192: F, t5195: F) -> (F, F, F, F, F) {
    let t5198 = t221 * t5196 * t1307;
    let t5202 = t118 * t794 * t1799;
    let t5203 = t3739 * t5202;
    let t5206 = t210 * t214 * t5187;
    let t5210 = t3725 + F::cast_from(0.38888888888888888888e-2_f64) * t3727 + t3731 + F::cast_from(0.38888888888888888887e-2_f64) * t5192 + F::cast_from(0.49999999999999999998e-2_f64) * t5195 * t5198 + F::cast_from(0.8333333333333333333e-3_f64) * t5203 - F::cast_from(0.16666666666666666666e-2_f64) * t1315 * t5206 + F::cast_from(0.83333333333333333332e-3_f64) * t3742 - t3751;
    (t5198, t5202, t5203, t5206, t5210)
}
