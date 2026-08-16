//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1279/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1279(t3886: f64, t1109: f64, t1115: f64, t12130: f64, t12232: f64, t13112: f64, t13187: f64, t2409: f64, t2416: f64, t2501: f64, t3055: f64, t3207: f64, t335: f64, t338: f64, t35000: f64, t353: f64, t35941: f64, t3780: f64, t3921: f64, t43290: f64, t44091: f64, t44093: f64, t44118: f64, t44131: f64, t44138: f64, t829: f64, t830: f64, t9885: f64, t9899: f64) -> f64 {
    let t50499 = t3886 * t3886;
    let t50514 = 35.0_f64 / 12.0_f64 * t35941 + t12130 * t829 * t830 * t2501 * t3780 / 8.0_f64 - t3921 * t9885 / 8.0_f64 + 7.0_f64 / 12.0_f64 * t44091 + 7.0_f64 / 24.0_f64 * t44093 - t3055 * t829 * t830 * t12232 * t1109 / 16.0_f64 - t3921 * t9899 / 16.0_f64 + t35000 * t13112 / 4.0_f64 + t335 * t338 * t353 * t2416 * t50499 / 16.0_f64 + 3.0_f64 / 4.0_f64 * t3207 * t2409 * t2501 * t13187 - 7.0_f64 / 36.0_f64 * t44118 - t1115 * t43290 / 12.0_f64 + 7.0_f64 / 12.0_f64 * t44131 + 7.0_f64 / 12.0_f64 * t44138;
    t50514
}
