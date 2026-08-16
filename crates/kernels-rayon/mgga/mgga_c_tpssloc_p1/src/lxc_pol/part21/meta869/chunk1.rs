//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3183/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3183(t1227: f64, t13969: f64, t18345: f64, t1174: f64, t1177: f64, t18943: f64, t3536: f64, t3555: f64, t52872: f64, t52875: f64, t55723: f64, t63294: f64, t63298: f64, t63302: f64, t65992: f64, t65994: f64, t65996: f64, t65998: f64, t66001: f64, t66015: f64, t66024: f64, t974: f64) -> f64 {
    let t66027 = t1227 * t13969 * t18345;
    let t66029 = -t65992 / 216.0_f64 - t65994 / 216.0_f64 + t65996 / 1152.0_f64 + t65998 / 1152.0_f64 - t66001 / 216.0_f64 - t1174 * t1177 * t63294 / 72.0_f64 - t1174 * t1177 * t63298 / 144.0_f64 - t1174 * t1177 * t63302 / 48.0_f64 + t3536 * t18943 / 1536.0_f64 + t66015 / 648.0_f64 - t1174 * t974 * t3555 * t55723 / 72.0_f64 + t52872 / 5184.0_f64 - t52875 / 1728.0_f64 + 5.0_f64 / 5184.0_f64 * t66024 + 5.0_f64 / 1728.0_f64 * t66027;
    t66029
}
