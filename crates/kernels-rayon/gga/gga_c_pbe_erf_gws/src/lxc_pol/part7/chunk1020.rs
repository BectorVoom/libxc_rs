//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1020/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1020(t18486: f64, t18488: f64, t18491: f64, t18494: f64, t18500: f64, t18502: f64, t18504: f64, t18506: f64, t389: f64, t404: f64, t7236: f64, t7271: f64) -> f64 {
    let t18512 = 1.0_f64 * t389 * (-0.21099166666666666667e1_f64 * t18486 + 0.202552e2_f64 * t18488 - 0.75019259259259259258e1_f64 * t18491 + 0.6564185185185185185e1_f64 * t18494 + 0.31003950617283950618e1_f64 * t7271 + 0.68258333333333333335e-1_f64 * t18500 - 0.10921333333333333333e1_f64 * t18502 + 0.12134814814814814815e1_f64 * t18504 + 0.10617962962962962963e1_f64 * t18506 + 0.13388493827160493828e1_f64 * t7236) * t404;
    t18512
}
