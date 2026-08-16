//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 609/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk609(t1022: f64, t3228: f64, t3227: f64, t1092: f64, t1121: f64, t2855: f64, t1096: f64, t2866: f64, t359: f64, t356: f64, t303: f64, t1087: f64, t1126: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3229 = t1022 * t3228;
    let t3230 = t3227 * t3229;
    let t3231 = t1092 * t3230;
    let t3233 = t2855 * t1121;
    let t3234 = t1096 * t3233;
    let t3235 = t1092 * t3234;
    let t3237 = t2866 * t359;
    let t3238 = t356 * t3237;
    let t3239 = t303 * t3238;
    let t3241 = t1087 * t1126;
    (t3229, t3230, t3231, t3233, t3234, t3235, t3237, t3238, t3239, t3241)
}
