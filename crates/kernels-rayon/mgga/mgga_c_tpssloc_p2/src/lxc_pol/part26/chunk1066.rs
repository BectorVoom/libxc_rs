//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 1066/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk1066(t2307: f64, t79: f64, t72: f64, t2244: f64, t605: f64, t2251: f64, t2241: f64, t2240: f64, t608: f64, t1864: f64, t645: f64, t1266: f64, t6534: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22530 = t79 * t2307;
    let t22531 = t72 * t22530;
    let t22534 = t605 * t2244;
    let t22537 = t605 * t2251;
    let t22546 = t72 * t79 * t2241;
    let t22549 = t2240 * t608;
    let t22550 = t1864 * t645;
    let t22561 = t1266 * t6534;
    (t22530, t22531, t22534, t22537, t22546, t22549, t22550, t22561)
}
