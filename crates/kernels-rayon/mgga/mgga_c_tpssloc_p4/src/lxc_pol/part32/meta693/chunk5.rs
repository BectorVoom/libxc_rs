//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2151/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2151(t22704: f64, t28134: f64, t80798: f64, t1985: f64, t1998: f64, t20009: f64, t214: f64, t1352: f64, t26331: f64, t6976: f64, t97011: f64, t1799: f64, t6637: f64, t6888: f64, t90809: f64) -> (f64, f64, f64, f64) {
    let t97049 = t22704 * t80798 * t28134;
    let t97055 = t1985 * t214 * t1998 * t20009;
    let t97059 = t26331 * t6976 * t97011 * t1352;
    let t97063 = t6888 * t6637 * t90809 * t1799;
    (t97049, t97055, t97059, t97063)
}
