//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 503/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk503(t4579: f64, t4581: f64, t126: f64, t66: f64, t41: f64, t85: f64, t1103: f64, t1104: f64, t167: f64, t1098: f64, t1758: f64, t1102: f64, t3253: f64, t3256: f64, t3258: f64, t3260: f64, t4563: f64, t4568: f64, t4572: f64, t4576: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4582 = t4579 * t4581;
    let t4585 = t126 * t66;
    let t4587 = t85 * t4585 * t41;
    let t4589 = t1103 * t1104 * t167;
    let t4592 = t1098 * t1758;
    let t4594 = -t3253 + 0.43802864444444444445e-3_f64 * t3256 + 0.98556445e-3_f64 * t3258 - 0.65704296666666666667e-3_f64 * t3260 + 0.43802864444444444445e-3_f64 * t4563 + 0.10950716111111111111e-2_f64 * t1102 * t4568 + 0.98556445e-3_f64 * t1102 * t4572 - 0.65704296666666666667e-3_f64 * t1102 * t4576 - 0.13140859333333333333e-2_f64 * t1102 * t4582 + 0.13140859333333333333e-2_f64 * t4587 * t4589 + 0.98556445e-3_f64 * t4592;
    (t4582, t4585, t4587, t4589, t4592, t4594)
}
