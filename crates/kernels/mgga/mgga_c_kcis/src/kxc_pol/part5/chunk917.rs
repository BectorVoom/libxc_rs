//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 917/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk917<F: Float>(t2792: F, t2627: F, t2540: F, t2534: F, t4527: F, t908: F, t2861: F, t5027: F, t5030: F, t1094: F, t4922: F, t1775: F, t9528: F, t341: F, t9368: F, t1017: F, t86: F, sigma0: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13031 = 3.0 * t2792;
    let t13034 = 3.0 * t2627;
    let t13043 = 6.0 * t2540;
    let t13044 = 6.0 * t2534;
    let t13095 = t4527 * t908;
    let t13096 = 2.0 * t13095;
    let t13101 = t2861 * t5027;
    let t13102 = 0.33163888888888888888e-2 * t13101;
    let t13103 = t2861 * t5030;
    let t13105 = t4922 * t1094;
    let t13106 = t13105 * sigma0;
    let t13122 = t9528 * t1775;
    let t13128 = t9368 * t341;
    let t13130 = t86 * t1017 * t13128;
    (t13031, t13034, t13043, t13044, t13096, t13101, t13102, t13103, t13105, t13106, t13122, t13130)
}
