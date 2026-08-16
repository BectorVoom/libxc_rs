//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 1288/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk1288(t1453: f64, t95: f64, t2331: f64, t64: f64, t91: f64, t29900: f64, t30168: f64, t656: f64, t9576: f64, t30176: f64, t29895: f64, t30159: f64) -> (f64, f64, f64, f64, f64) {
    let t110521 = t95 * t1453;
    let t110526 = t64 * t2331 * t91;
    let t110531 = 50.0_f64 / 27.0_f64 * t29900 * t30168;
    let t110532 = t9576 * t656;
    let t110533 = t110532 * t30176;
    let t110542 = 4.0_f64 / 3.0_f64 * t29895 * t30159;
    (t110521, t110526, t110531, t110533, t110542)
}
