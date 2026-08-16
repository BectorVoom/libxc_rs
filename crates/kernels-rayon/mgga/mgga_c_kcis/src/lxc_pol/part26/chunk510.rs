//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 510/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk510(t5439: f64, t5441: f64, t1471: f64, t1472: f64, t167: f64, t1098: f64, t1992: f64, t1102: f64, t3743: f64, t3744: f64, t3746: f64, t3748: f64, t4587: f64, t5423: f64, t5428: f64, t5432: f64, t5436: f64) -> (f64, f64, f64, f64) {
    let t5442 = t5439 * t5441;
    let t5446 = t1471 * t1472 * t167;
    let t5449 = t1098 * t1992;
    let t5451 = -t3743 + 0.43802864444444444445e-3_f64 * t3744 + 0.98556445e-3_f64 * t3746 - 0.65704296666666666667e-3_f64 * t3748 + 0.43802864444444444445e-3_f64 * t5423 + 0.10950716111111111111e-2_f64 * t1102 * t5428 + 0.98556445e-3_f64 * t1102 * t5432 - 0.65704296666666666667e-3_f64 * t1102 * t5436 - 0.13140859333333333333e-2_f64 * t1102 * t5442 - 0.13140859333333333333e-2_f64 * t4587 * t5446 + 0.98556445e-3_f64 * t5449;
    (t5442, t5446, t5449, t5451)
}
