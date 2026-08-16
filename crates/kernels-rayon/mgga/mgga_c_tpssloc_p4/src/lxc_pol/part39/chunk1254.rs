//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1254/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1254(t28: f64, t12072: f64, t1649: f64, t2: f64, t3672: f64, t1081: f64, t584: f64, t16: f64, t3231: f64, t3673: f64, t5142: f64, t5145: f64, t517: f64, zeta_threshold: f64) -> (f64, f64) {
    let t29 = t28 <= zeta_threshold;
    let t15952 = t12072 * t1649;
    let t15955 = t3672 * t2;
    let t15956 = t584 * t1081;
    let t15966 = piecewise3(t29, 0.0_f64, -8.0_f64 / 27.0_f64 * t15952 * t3673 - 16.0_f64 / 9.0_f64 * t15955 * t15956 + 4.0_f64 / 9.0_f64 * t5142 * t3231 - 8.0_f64 / 3.0_f64 * t517 * t584 + 8.0_f64 * t5145 * t16);
    (t15956, t15966)
}
