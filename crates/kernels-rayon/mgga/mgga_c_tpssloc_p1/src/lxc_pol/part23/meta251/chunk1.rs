//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 911/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk911(t4993: f64, t5024: f64, t1017: f64, t6163: f64, t1210: f64, t1207: f64, t372: f64, t479: f64, t471: f64) -> (f64, f64, f64, f64, f64) {
    let t18987 = t5024 * t4993;
    let t19024 = t6163 * t1017;
    let t19025 = t1210 * t19024;
    let t19026 = t1207 * t19025;
    let t19031 = t6163 * t372;
    let t19032 = t479 * t19031;
    let t19033 = t471 * t19032;
    (t18987, t19025, t19026, t19032, t19033)
}
