//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 240/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk240<F: Float>(t1010: F, t1053: F, t167: F, t220: F, t247: F, t242: F, t819: F, t821: F, t825: F, t827: F, t250: F) -> (F, F, F, F, F, F, F) {
    let t1054 = t1010 * t1053;
    let t1055 = t220 * t167;
    let t1070 = t247 * t247;
    let t1071 = F::cast_from(1.0_f64) / t1070;
    let t1072 = t242 * t1071;
    let t1077 = -F::cast_from(0.1176575e1_f64) * t819 - F::cast_from(0.516475e0_f64) * t821 - F::cast_from(0.2103875e0_f64) * t825 - F::cast_from(0.104195e0_f64) * t827;
    let t1078 = F::cast_from(1.0_f64) / t250;
    (t1054, t1055, t1070, t1071, t1072, t1077, t1078)
}
