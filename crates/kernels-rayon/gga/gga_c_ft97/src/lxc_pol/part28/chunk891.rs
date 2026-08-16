//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 891/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk891(t1017: f64, t7407: f64, t574: f64, t605: f64, t1901: f64, t33146: f64, t35073: f64, t35076: f64, t35080: f64, t35084: f64, t35087: f64, t35091: f64, t35095: f64, t35099: f64, t35103: f64, t35107: f64, t446: f64) -> (f64, f64, f64) {
    let t35110 = t7407 * t1017;
    let t35112 = t574 * t605 * t35110;
    let t35115 = 2.0_f64 / 3.0_f64 * t446 * t35073 + 2.0_f64 / 9.0_f64 * t1901 * t35076 - 4.0_f64 / 3.0_f64 * t1901 * t35080 - 2.0_f64 / 9.0_f64 * t1901 * t35084 + 2.0_f64 / 9.0_f64 * t1901 * t35087 + t1901 * t35091 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t35095 + t33146 + 2.0_f64 / 3.0_f64 * t446 * t35099 + 4.0_f64 / 3.0_f64 * t446 * t35103 - 2.0_f64 * t446 * t35107 + t446 * t35112 / 3.0_f64;
    (t35110, t35112, t35115)
}
