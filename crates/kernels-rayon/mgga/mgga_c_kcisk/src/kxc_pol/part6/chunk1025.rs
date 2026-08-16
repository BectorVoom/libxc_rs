//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 1025/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk1025(t1349: f64, t1391: f64, t14056: f64, t14059: f64, t14062: f64, t14085: f64, t14093: f64, t158: f64, t20752: f64, t20754: f64, t20763: f64, t30153: f64, t30158: f64, t30838: f64, t30852: f64, t3819: f64) -> f64 {
    let t30855 = -0.7026e-2_f64 * t158 * t30838 + 0.11955719325063177623e-1_f64 * t1349 * t30158 - 0.5179538907796306876e-4_f64 * t1391 * t30158 + 0.71734315950379065738e-1_f64 * t14093 * t30153 - 0.62154466893555682512e-3_f64 * t14085 * t30153 + t14056 + t14059 - t14062 + 0.10566666666666666666e-1_f64 * t20752 + 0.117630625e-3_f64 * t20754 - 0.32788e-1_f64 * t20763 - 0.71734315950379065738e-1_f64 * t3819 * t30852;
    t30855
}
