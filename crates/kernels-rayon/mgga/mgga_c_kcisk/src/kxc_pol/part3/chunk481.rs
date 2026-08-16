//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 481/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk481(t1163: f64, t3764: f64, t1340: f64, t1339: f64, t1341: f64, t3579: f64, t1329: f64, t3489: f64, t3491: f64, t3497: f64, t3505: f64, t3510: f64, t3514: f64, t3735: f64, t3740: f64, t3745: f64, t3749: f64, t3752: f64, t3756: f64, t3762: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t3765 = t3764 * t1163;
    let t3766 = t1340 * t3765;
    let t3767 = t1339 * t3766;
    let t3769 = t1341 * t3579;
    let t3770 = t1340 * t3769;
    let t3771 = t1339 * t3770;
    let t3773 = 0.22109259259259259258e-2_f64 * t3489 - 0.386e0_f64 * t3491 * t1329 - 0.88437037037037037034e-2_f64 * t3497 + 0.16581944444444444444e-2_f64 * t3505 - 0.49745833333333333332e-2_f64 * t3510 + 0.33163888888888888888e-2_f64 * t3514 - 0.24872916666666666666e-2_f64 * t3735 - 0.33163888888888888888e-2_f64 * t3740 - 0.33163888888888888888e-2_f64 * t3745 + 0.22109259259259259258e-2_f64 * t3749 + 0.33163888888888888888e-2_f64 * t3752 + 0.16581944444444444444e-2_f64 * t3756 + 0.27636574074074074073e-2_f64 * t3762 - 0.88437037037037037034e-2_f64 * t3767 - 0.33163888888888888888e-2_f64 * t3771;
    (t3765, t3766, t3767, t3769, t3770, t3771, t3773)
}
