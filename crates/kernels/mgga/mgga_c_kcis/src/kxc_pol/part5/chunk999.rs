//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 999/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk999<F: Float>(t4527: F, t908: F, t2861: F, t5027: F, t5030: F, t1094: F, t4922: F, t1775: F, t9528: F, t341: F, t9368: F, t1017: F, t86: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t13095 = t4527 * t908;
    let t13096 = F::new(2.0) * t13095;
    let t13101 = t2861 * t5027;
    let t13102 = F::cast_from(0.33163888888888888888e-2_f64) * t13101;
    let t13103 = t2861 * t5030;
    let t13105 = t4922 * t1094;
    let t13106 = t13105 * sigma0;
    let t13122 = t9528 * t1775;
    let t13128 = t9368 * t341;
    let t13130 = t86 * t1017 * t13128;
    (t13096, t13101, t13102, t13103, t13105, t13106, t13122, t13130)
}
