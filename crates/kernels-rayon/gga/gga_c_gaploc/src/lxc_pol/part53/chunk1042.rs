//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 1042/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk1042(t40591: f64, t40596: f64, t40599: f64, t42974: f64, t42978: f64, t42980: f64, t42984: f64, t42985: f64, t42988: f64, t42991: f64, t42998: f64, t43006: f64, t43014: f64, t43017: f64, t43019: f64, t43023: f64, t47616: f64, t47619: f64) -> f64 {
    let t51007 = -t42974 - t42978 - t42980 - 0.44861403009162974988e-2_f64 * t40591 + 0.76905262301422242835e-2_f64 * t40596 - 0.38452631150711121417e-2_f64 * t40599 + t42984 + 0.1281754371690370714e-2_f64 * t42985 + 0.1281754371690370714e-2_f64 * t42988 + 0.1281754371690370714e-2_f64 * t42991 - t47616 + t47619 - t42998 + t43006 - t43014 - t43017 + t43019 - t43023;
    t51007
}
