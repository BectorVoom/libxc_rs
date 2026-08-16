//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 537/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk537(t103: f64, t2354: f64, t100: f64, t2336: f64, t2343: f64, t2346: f64, t2351: f64, t657: f64, t660: f64, t92: f64, t96: f64) -> (f64, f64) {
    let t2355 = t103 * t2354;
    let t2358 = 40.0_f64 / 9.0_f64 * t2336 * t96 - 50.0_f64 / 9.0_f64 * t657 * t660 + 10.0_f64 / 9.0_f64 * t92 * t2343 + 5.0_f64 / 3.0_f64 * t92 * t2346 + 10.0_f64 / 9.0_f64 * t100 * t2351 + 5.0_f64 / 3.0_f64 * t100 * t2355;
    (t2355, t2358)
}
