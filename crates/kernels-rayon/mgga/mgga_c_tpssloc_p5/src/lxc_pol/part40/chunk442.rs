//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 442/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk442(t109: f64, t1453: f64, t656: f64, t64: f64, t654: f64) -> (f64, f64) {
    let t110 = 1.0_f64 < t109;
    let t1454 = t656 * t1453;
    let t1458 = piecewise3(t110, 0.0_f64, -t654 - t64 * t1454 / 8.0_f64);
    (t1454, t1458)
}
