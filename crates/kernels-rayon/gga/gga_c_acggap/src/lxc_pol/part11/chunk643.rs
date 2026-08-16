//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 643/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk643(t288: f64, t5042: f64, t1381: f64, t682: f64, t2806: f64, t2812: f64, t2979: f64, t2983: f64, t2989: f64, t2995: f64, t5030: f64, t5031: f64, t5033: f64, t5035: f64, t5037: f64, t5038: f64, t5039: f64, t5041: f64) -> f64 {
    let t5043 = t5042 * t288;
    let t5044 = 0.11696447245269292414e1_f64 * t5043;
    let t5045 = t1381 * t682;
    let t5046 = 0.5848223622634646207e0_f64 * t5045;
    let t5047 = t5030 - t2979 - t2983 - t5031 + t5033 + t5035 - t5037 - t2989 + t2806 - t2812 + t5038 + t2995 + t5039 - t5041 - t5044 - t5046;
    t5047
}
