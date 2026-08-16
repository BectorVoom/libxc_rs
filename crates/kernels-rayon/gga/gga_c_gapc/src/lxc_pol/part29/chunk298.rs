//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 298/1311 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk298(t1092: f64, t1093: f64, t1070: f64, t1075: f64, t1079: f64, t1082: f64, t1090: f64) -> f64 {
    let t1094 = t1092 * t1093;
    let t1096 = 0.13900948042322754167e-2_f64 * t1070 + 0.50602213541666666669e-5_f64 * t1075 - 0.86880925264517213544e-4_f64 * t1079 - 0.11594181388521408695e-4_f64 * t1082 - 0.42205124476153752644e-7_f64 * t1090 + 0.72463633678258804342e-6_f64 * t1094;
    t1096
}
