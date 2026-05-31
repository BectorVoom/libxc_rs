//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 531/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk531<F: Float>(t1572: F, t4332: F, t3793: F, t3879: F, t3795: F, t3799: F, t3803: F, t3807: F, t3829: F, t3831: F, t3874: F, t3876: F, t3881: F, t3885: F, t3888: F, t3891: F) -> (F, F, F, F) {
    let t4333 = t4332 * t1572;
    let t4338 = F::cast_from(0.68863333333333333333e0_f64) * t3793;
    let t4345 = F::cast_from(0.17365833333333333333e0_f64) * t3879;
    let t4350 = -F::cast_from(0.17648625e1_f64) * t3829 + F::cast_from(0.3529725e1_f64) * t3831 + t4338 + F::cast_from(0.34431666666666666666e0_f64) * t3795 - F::cast_from(0.34431666666666666667e0_f64) * t3799 + F::cast_from(0.103295e1_f64) * t3803 - F::cast_from(0.516475e0_f64) * t3807 + F::cast_from(0.31558125e0_f64) * t3874 + F::cast_from(0.6311625e0_f64) * t3876 + t4345 + F::cast_from(0.13892666666666666667e0_f64) * t3881 - F::cast_from(0.34731666666666666667e-1_f64) * t3885 + F::cast_from(0.20839e0_f64) * t3888 - F::cast_from(0.104195e0_f64) * t3891;
    (t4333, t4338, t4345, t4350)
}
