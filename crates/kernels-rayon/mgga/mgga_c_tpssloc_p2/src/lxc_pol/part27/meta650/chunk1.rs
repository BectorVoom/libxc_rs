//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2263/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2263(t1868: f64, t2363: f64, t5107: f64, t652: f64, t6534: f64, t22574: f64, t56198: f64, t8643: f64, t26162: f64, t57802: f64, t22597: f64, t7685: f64) -> (f64, f64, f64, f64, f64) {
    let t90044 = t1868 * t2363;
    let t90051 = 4.0_f64 * t652 * t5107 * t6534;
    let t90059 = 6.0_f64 * t22574 * t8643 * t56198;
    let t90062 = 6.0_f64 * t22574 * t26162 * t57802;
    let t90064 = 6.0_f64 * t7685 * t22597;
    (t90044, t90051, t90059, t90062, t90064)
}
