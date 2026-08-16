//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 933/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk933(t1897: f64, t1901: f64, t42980: f64, t42981: f64, t42982: f64, t42983: f64, t42984: f64, t42986: f64, t42989: f64, t42992: f64, t42993: f64, t42998: f64, t42999: f64, t43003: f64, t43006: f64, t43010: f64, t43014: f64, t43017: f64, t43019: f64, t43023: f64) -> f64 {
    let t43024 = -t42980 - t42981 + t42982 - t42983 + t42984 + t42986 + t42989 + t42992 + 0.76905262301422242837e-2_f64 * t1897 * t1901 * t42993 - t42998 + 0.41016139894091862845e-1_f64 * t42999 + 0.30762104920568897134e-1_f64 * t43003 + t43006 - 0.34180116578409885704e-2_f64 * t43010 - t43014 - t43017 + t43019 - t43023;
    t43024
}
