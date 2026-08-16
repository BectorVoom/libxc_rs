//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 856/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk856(t6547: f64, t8538: f64, t30697: f64, t30704: f64, t30721: f64, t2047: f64, t214: f64) -> (f64, f64, f64, f64, f64) {
    let t31349 = t6547 * t8538;
    let t31350 = 0.19190897446562641759e-1_f64 * t31349;
    let t31353 = 0.11304371706359309439e-1_f64 * t30697;
    let t31355 = 0.26915170729426927235e-3_f64 * t30704;
    let t31359 = 7.0_f64 / 1152.0_f64 * t30721;
    let t31366 = t214 * t2047;
    (t31350, t31353, t31355, t31359, t31366)
}
