//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1329/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1329(t1484: f64, t193: f64, t202: f64, t20800: f64, t2522: f64, t39593: f64, t41254: f64, t4310: f64, t67112: f64, t75950: f64, t75951: f64, t75952: f64, t75978: f64, t76017: f64, t76018: f64, t76020: f64, t76024: f64, t76025: f64, t76497: f64, t76532: f64, t766: f64, t870: f64) -> f64 {
    let t76543 = -t39593 + t75950 + t75951 - t75952 + 3.0_f64 * t193 * t766 * t75978 + t193 * t202 * (t76497 + t76532) * t870 + 12.0_f64 * t2522 * t4310 * t20800 + t76017 + 12.0_f64 * t2522 * t67112 * t1484 + t41254 - t76018 + t76020 + t76024 + t76025;
    t76543
}
