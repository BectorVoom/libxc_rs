//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 977/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk977<F: Float>(t1464: F, t547: F, t528: F, t5975: F, t145: F, t4562: F, t164: F, t4551: F, t18041: F, t18053: F, t18054: F, t18057: F, t18061: F, t18065: F, t18067: F, t18069: F, t18072: F, t18073: F, t18077: F, t18079: F, t18080: F, t18082: F, t18084: F, t18086: F) -> (F, F) {
    let t18089 = F::cast_from(0.75612977335538682803e0_f64) * t1464 * t547;
    let t18091 = F::cast_from(0.12602162889256447134e0_f64) * t528 * t5975;
    let t18092 = t145 * t4562;
    let t18093 = t18092 * t164;
    let t18095 = t4551 * t547;
    let t18097 = -F::cast_from(0.12602162889256447134e0_f64) * t18041 - t18053 - F::cast_from(0.31505407223141117834e-1_f64) * t18054 * t164 - F::cast_from(0.12602162889256447134e0_f64) * t18057 + F::cast_from(0.35922702030763827281e-1_f64) * t18061 + F::cast_from(0.35124419763413520009e0_f64) * t18065 - t18067 - F::cast_from(0.47461239486605618761e-3_f64) * t18069 - t18072 + F::cast_from(0.37806488667769341401e0_f64) * t18073 + t18077 - t18079 - F::cast_from(0.189032443338846707e0_f64) * t18080 - F::cast_from(0.37806488667769341401e0_f64) * t18082 - F::cast_from(0.75612977335538682804e0_f64) * t18084 + F::cast_from(0.75612977335538682803e0_f64) * t18086 + t18089 + t18091 + F::cast_from(0.12602162889256447134e0_f64) * t18093 + F::cast_from(0.37806488667769341401e0_f64) * t18095;
    (t18092, t18097)
}
