//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 684/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk684<F: Float>(t468: F, t3862: F, t3899: F, t3793: F, t3795: F, t3799: F, t3803: F, t3807: F, t482: F, t1341: F, t45: F, t1346: F, t478: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3900 = t468 * t468;
    let t3901 = F::new(1.0) / t3900;
    let t3902 = t3862 * t3901;
    let t3904 = F::new(0.16081824322151104822e2) * t3899 * t3902;
    let t3905 = F::new(0.12361111111111111111e-1) * t3793;
    let t3910 = t3905 + F::new(0.61805555555555555556e-2) * t3795 - F::new(0.61805555555555555555e-2) * t3799 + F::new(0.18541666666666666667e-1) * t3803 - F::new(0.92708333333333333333e-2) * t3807;
    let t3911 = t3910 * t482;
    let t3914 = t45 * t1341;
    let t3917 = t1346 * t478;
    (t3900, t3901, t3902, t3904, t3905, t3910, t3911, t3914, t3917)
}
