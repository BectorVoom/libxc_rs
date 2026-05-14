//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 892/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk892<F: Float>(t1538: F, t4455: F, t1536: F, t4463: F, t1543: F, t3716: F, t3725: F, t1210: F, t12974: F, t12922: F, t12927: F, t12929: F, t12931: F, t12933: F, t12948: F, t12954: F, t12959: F, t12985: F, t12989: F) -> (F, F, F, F, F) {
    let t14817 = t1538 * t4455;
    let t14821 = t4455 * t4463 * t1536;
    let t14824 = t1543 * t3716;
    let t14827 = t3716 * t3725;
    let t14828 = t14827 * t1210;
    let t14831 = 0.53272592592592592592e-1 * t12974;
    let t14842 = -t14831 - 0.2283111111111111111e-1 * t12929 + 0.11415555555555555555e-1 * t12933 - 0.34246666666666666665e-1 * t12948 + 0.17123333333333333333e-1 * t12931 - 0.19025925925925925925e-1 * t12922 + 0.68493333333333333331e-1 * t12954 - 0.34246666666666666665e-1 * t12985 - 0.10274e0 * t12959 + 0.10274e0 * t12989 - 0.17123333333333333333e-1 * t12927;
    (t14817, t14821, t14824, t14828, t14842)
}
