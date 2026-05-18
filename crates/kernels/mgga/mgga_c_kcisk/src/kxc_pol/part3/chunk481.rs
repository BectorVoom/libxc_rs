//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 481/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk481<F: Float>(t1163: F, t3764: F, t1340: F, t1339: F, t1341: F, t3579: F, t1329: F, t3489: F, t3491: F, t3497: F, t3505: F, t3510: F, t3514: F, t3735: F, t3740: F, t3745: F, t3749: F, t3752: F, t3756: F, t3762: F) -> (F, F, F, F, F, F, F) {
    let t3765 = t3764 * t1163;
    let t3766 = t1340 * t3765;
    let t3767 = t1339 * t3766;
    let t3769 = t1341 * t3579;
    let t3770 = t1340 * t3769;
    let t3771 = t1339 * t3770;
    let t3773 = F::new(0.22109259259259259258e-2) * t3489 - F::new(0.386e0) * t3491 * t1329 - F::new(0.88437037037037037034e-2) * t3497 + F::new(0.16581944444444444444e-2) * t3505 - F::new(0.49745833333333333332e-2) * t3510 + F::new(0.33163888888888888888e-2) * t3514 - F::new(0.24872916666666666666e-2) * t3735 - F::new(0.33163888888888888888e-2) * t3740 - F::new(0.33163888888888888888e-2) * t3745 + F::new(0.22109259259259259258e-2) * t3749 + F::new(0.33163888888888888888e-2) * t3752 + F::new(0.16581944444444444444e-2) * t3756 + F::new(0.27636574074074074073e-2) * t3762 - F::new(0.88437037037037037034e-2) * t3767 - F::new(0.33163888888888888888e-2) * t3771;
    (t3765, t3766, t3767, t3769, t3770, t3771, t3773)
}
