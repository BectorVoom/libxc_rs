//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1210/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1210(t3802: f64, t20133: f64, t3703: f64, t3717: f64, t2083: f64, t3776: f64, t1109: f64, t1115: f64, t1118: f64, t11375: f64, t1162: f64, t12182: f64, t13121: f64, t13290: f64, t20138: f64, t2409: f64, t326: f64, t35000: f64, t35057: f64, t353: f64, t3737: f64, t3912: f64, t43323: f64, t43549: f64, t43671: f64, t4386: f64, t46763: f64, t46867: f64, t825: f64, t831: f64, t833: f64, t859: f64, t8599: f64, t8629: f64, t8793: f64, t9241: f64) -> (f64, f64, f64, f64, f64) {
    let t49063 = t3802 * t3802;
    let t49064 = t20133 * t49063;
    let t49087 = t3703 * t3717;
    let t49092 = t2083 * t3776;
    let t49102 = -t1115 * t43323 + t1115 * t46763 / 24.0_f64 + t326 * t49064 * t20138 * t833 / 96.0_f64 - 7.0_f64 / 36.0_f64 * t43549 - t11375 * t4386 * t353 * t1118 * t13290 / 6.0_f64 - 3.0_f64 / 8.0_f64 * t8629 * t8599 * t353 * t3737 * t1109 + t8793 * t46867 / 4.0_f64 + t35057 * t12182 / 4.0_f64 + t35000 * t13121 / 8.0_f64 - 7.0_f64 / 6.0_f64 * t43671 - 3.0_f64 / 2.0_f64 * t9241 * t2409 * t831 * t49087 + t3912 * t49092 * t825 * t833 / 32.0_f64 - t11375 * t859 * t353 * t1162 * t13290 / 12.0_f64;
    (t49063, t49064, t49087, t49092, t49102)
}
