//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1211/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1211(t1109: f64, t1115: f64, t11375: f64, t1144: f64, t1161: f64, t13119: f64, t13121: f64, t2409: f64, t3066: f64, t34922: f64, t353: f64, t3721: f64, t3722: f64, t3886: f64, t3887: f64, t3917: f64, t43451: f64, t43643: f64, t43734: f64, t44025: f64, t44149: f64, t46707: f64, t46862: f64, t47050: f64, t859: f64, t8629: f64, t8793: f64, t9296: f64, t9885: f64) -> f64 {
    let t49147 = -t8629 * t859 * t353 * t3722 * t1109 / 8.0_f64 + t34922 * t13121 / 12.0_f64 + 3.0_f64 / 4.0_f64 * t8793 * t44025 + t11375 * t859 * t353 * t43451 * t1109 / 16.0_f64 + t8629 * t859 * t1144 * t13119 / 8.0_f64 + t8793 * t43643 / 4.0_f64 + 7.0_f64 / 24.0_f64 * t43734 - t3917 * t9885 / 8.0_f64 - 3.0_f64 / 8.0_f64 * t3066 * t2409 * t9296 * t3721 * t3886 + 3.0_f64 / 4.0_f64 * t1115 * t46707 + t8629 * t859 * t353 * t47050 * t1161 / 24.0_f64 + t8629 * t859 * t353 * t3887 * t1109 / 16.0_f64 - t8793 * t44149 / 2.0_f64 - 3.0_f64 / 4.0_f64 * t8793 * t46862;
    t49147
}
