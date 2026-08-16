//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 964/1056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk964(t31286: f64, t23893: f64, t24465: f64, t23896: f64, t55571: f64, t8657: f64, t1873: f64, t23917: f64, t3941: f64, t6534: f64, t7056: f64, t45560: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114500 = 54.0_f64 * t31286;
    let t114513 = 54.0_f64 * t24465 * t23893;
    let t114515 = 27.0_f64 * t24465 * t23896;
    let t114517 = 27.0_f64 * t55571 * t8657;
    let t114520 = 27.0_f64 * t3941 * t23917 * t1873;
    let t114525 = 54.0_f64 * t3941 * t7056 * t6534;
    let t114527 = 27.0_f64 * t45560 * t8657;
    (t114500, t114513, t114515, t114517, t114520, t114525, t114527)
}
