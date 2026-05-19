//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 677/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk677<F: Float>(t3820: F, t513: F, t1317: F, t1416: F, t3781: F, t3809: F, t3793: F, t3795: F, t3799: F, t3803: F, t3807: F, t1319: F, t1410: F, t456: F) -> (F, F, F, F, F, F, F) {
    let t3821 = t3820 * t513;
    let t3824 = t1317 * t1416;
    let t3829 = t3820 * t3781;
    let t3831 = t1317 * t3809;
    let t3833 = F::cast_from(0.55033333333333333333e-2_f64) * t3793;
    let t3838 = -F::new(0.991e-2) * t3829 + F::new(0.1982e-1) * t3831 + t3833 + F::cast_from(0.27516666666666666666e-2_f64) * t3795 - F::cast_from(0.27516666666666666667e-2_f64) * t3799 + F::new(0.8255e-2) * t3803 - F::new(0.41275e-2) * t3807;
    let t3841 = -t3821 * t3781 / F::new(8.0) + t3824 * t1319 / F::new(2.0) + t1410 * t3809 / F::new(4.0) + t456 * t3838 / F::new(2.0);
    (t3821, t3824, t3829, t3831, t3833, t3838, t3841)
}
