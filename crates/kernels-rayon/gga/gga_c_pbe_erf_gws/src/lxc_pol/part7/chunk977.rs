//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 977/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk977(t1464: f64, t547: f64, t528: f64, t5975: f64, t145: f64, t4562: f64, t164: f64, t4551: f64, t18041: f64, t18053: f64, t18054: f64, t18057: f64, t18061: f64, t18065: f64, t18067: f64, t18069: f64, t18072: f64, t18073: f64, t18077: f64, t18079: f64, t18080: f64, t18082: f64, t18084: f64, t18086: f64) -> (f64, f64) {
    let t18089 = 0.75612977335538682803e0_f64 * t1464 * t547;
    let t18091 = 0.12602162889256447134e0_f64 * t528 * t5975;
    let t18092 = t145 * t4562;
    let t18093 = t18092 * t164;
    let t18095 = t4551 * t547;
    let t18097 = -0.12602162889256447134e0_f64 * t18041 - t18053 - 0.31505407223141117834e-1_f64 * t18054 * t164 - 0.12602162889256447134e0_f64 * t18057 + 0.35922702030763827281e-1_f64 * t18061 + 0.35124419763413520009e0_f64 * t18065 - t18067 - 0.47461239486605618761e-3_f64 * t18069 - t18072 + 0.37806488667769341401e0_f64 * t18073 + t18077 - t18079 - 0.189032443338846707e0_f64 * t18080 - 0.37806488667769341401e0_f64 * t18082 - 0.75612977335538682804e0_f64 * t18084 + 0.75612977335538682803e0_f64 * t18086 + t18089 + t18091 + 0.12602162889256447134e0_f64 * t18093 + 0.37806488667769341401e0_f64 * t18095;
    (t18092, t18097)
}
