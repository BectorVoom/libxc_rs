//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1270/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1270(t18375: f64, t5002: f64, t1730: f64, t19032: f64, t1017: f64, t1207: f64, t1210: f64, t22173: f64, t372: f64, t471: f64, t479: f64, t15507: f64, t19095: f64) -> (f64, f64, f64, f64, f64) {
    let t72366 = t5002 * t18375;
    let t72384 = t1730 * t19032;
    let t72389 = t1207 * t1210 * t22173 * t1017;
    let t72398 = t471 * t479 * t22173 * t372;
    let t72403 = t15507 * t19095;
    (t72366, t72384, t72389, t72398, t72403)
}
