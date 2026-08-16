//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1005/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1005(t1538: f64, t4455: f64, t1536: f64, t4463: f64, t1543: f64, t3716: f64, t3725: f64, t1210: f64, t12974: f64, t12922: f64, t12927: f64, t12929: f64, t12931: f64, t12933: f64, t12948: f64, t12954: f64, t12959: f64, t12985: f64, t12989: f64) -> (f64, f64, f64, f64, f64) {
    let t14817 = t1538 * t4455;
    let t14821 = t4455 * t4463 * t1536;
    let t14824 = t1543 * t3716;
    let t14827 = t3716 * t3725;
    let t14828 = t14827 * t1210;
    let t14831 = 0.53272592592592592592e-1_f64 * t12974;
    let t14842 = -t14831 - 0.2283111111111111111e-1_f64 * t12929 + 0.11415555555555555555e-1_f64 * t12933 - 0.34246666666666666665e-1_f64 * t12948 + 0.17123333333333333333e-1_f64 * t12931 - 0.19025925925925925925e-1_f64 * t12922 + 0.68493333333333333331e-1_f64 * t12954 - 0.34246666666666666665e-1_f64 * t12985 - 0.10274e0_f64 * t12959 + 0.10274e0_f64 * t12989 - 0.17123333333333333333e-1_f64 * t12927;
    (t14817, t14821, t14824, t14828, t14842)
}
