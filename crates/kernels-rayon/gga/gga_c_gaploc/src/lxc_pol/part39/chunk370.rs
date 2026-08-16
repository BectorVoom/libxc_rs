//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 370/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk370(t3066: f64, t568: f64, t1040: f64, t1044: f64, t1049: f64, t1998: f64, t2004: f64, t2009: f64, t2049: f64, t2103: f64, t2194: f64, t2197: f64, t2639: f64, t2721: f64, t2725: f64, t3015: f64, t3019: f64, t3022: f64, t3025: f64, t3028: f64, t3032: f64, t3035: f64, t3040: f64, t3043: f64, t3046: f64, t3050: f64, t3055: f64, t3061: f64, t317: f64, t780: f64, t797: f64, t807: f64, t813: f64, t833: f64) -> f64 {
    let t3067 = t568 * t3066;
    let t3072 = 0.71500979903700853338e0_f64 * t2103 * t3015 - 0.46011511144704899612e1_f64 * t813 * t3019 + 0.11502877786176224903e2_f64 * t833 * t3022 - 0.10725146985555128001e1_f64 * t3025 * t2639 + 0.23005755572352449806e1_f64 * t807 * t3028 - 0.23005755572352449806e1_f64 * t1998 * t3032 - 0.35750489951850426669e0_f64 * t3035 * t2009 + 0.35750489951850426669e0_f64 * t780 * t3040 + 0.35750489951850426669e0_f64 * t2004 * t3043 + 0.35750489951850426669e0_f64 * t3046 * t317 + 0.35750489951850426669e0_f64 * t3050 * t317 - 0.35750489951850426669e0_f64 * t2049 * t1040 - 0.35750489951850426669e0_f64 * t797 * t3055 - 0.23005755572352449806e1_f64 * t2194 * t1044 - 0.23005755572352449806e1_f64 * t813 * t3061 + 0.23005755572352449806e1_f64 * t2197 * t1049 + 0.23005755572352449806e1_f64 * t833 * t3067 - 0.19171462976960374838e0_f64 * t2721 + 0.42603251059911944084e-1_f64 * t2725;
    t3072
}
