//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 678/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk678(t1482: f64, t3841: f64, t542: f64, t1360: f64, t3793: f64, t3795: f64, t3799: f64, t3803: f64, t3807: f64, t469: f64, t1311: f64, t1315: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3842 = t1482 * t3841;
    let t3843 = t542 * t3842;
    let t3846 = t1360 * t1360;
    let t3848 = 0.23744444444444444444e-1_f64 * t3793;
    let t3853 = t3848 + 0.11872222222222222222e-1_f64 * t3795 - 0.11872222222222222222e-1_f64 * t3799 + 0.35616666666666666666e-1_f64 * t3803 - 0.17808333333333333333e-1_f64 * t3807;
    let t3855 = 0.62182e-1_f64 * t3853 * t469;
    let t3856 = t1311 * t1315;
    (t3842, t3843, t3846, t3848, t3853, t3855, t3856)
}
