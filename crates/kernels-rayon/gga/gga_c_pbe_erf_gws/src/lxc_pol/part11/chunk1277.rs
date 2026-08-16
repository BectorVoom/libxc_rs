//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1277/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1277(t1105: f64, t1109: f64, t1115: f64, t12111: f64, t13142: f64, t13174: f64, t13182: f64, t13217: f64, t2376: f64, t2401: f64, t2408: f64, t2409: f64, t2501: f64, t3047: f64, t3052: f64, t3055: f64, t3207: f64, t335: f64, t338: f64, t353: f64, t3703: f64, t3717: f64, t3722: f64, t3733: f64, t376: f64, t3772: f64, t3886: f64, t3896: f64, t3907: f64, t3921: f64, t43451: f64, t4386: f64, t46656: f64, t46667: f64, t49955: f64, t829: f64, t830: f64, t8589: f64, t8629: f64, t9815: f64) -> f64 {
    let t50440 = t2408 * t2409 * t2376 * t3717 * t3886 / 8.0_f64 - 3.0_f64 / 4.0_f64 * t3207 * t2409 * t8589 * t13182 - 3.0_f64 / 8.0_f64 * t3207 * t2409 * t2376 * t3703 * t3886 + t8629 * t4386 * t353 * t3896 * t1109 / 8.0_f64 + t8629 * t4386 * t353 * t43451 * t1105 / 12.0_f64 + t335 * t338 * t3907 * t3722 / 8.0_f64 + 3.0_f64 / 16.0_f64 * t2401 * t338 * t353 * t376 * t49955 - t1115 * t46656 / 4.0_f64 - t3055 * t829 * t830 * t2501 * t3772 / 24.0_f64 - t9815 * t13217 / 32.0_f64 + t3921 * t12111 / 8.0_f64 - t46667 * t3733 / 32.0_f64 - t13142 * t3052 / 12.0_f64 - t13174 * t3047 / 24.0_f64;
    t50440
}
