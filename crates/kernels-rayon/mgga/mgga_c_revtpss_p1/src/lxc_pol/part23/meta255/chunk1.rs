//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1442/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1442(t550: f64, t816: f64, t9707: f64, t1379: f64, t2689: f64, t3952: f64, t547: f64, t9646: f64, t2236: f64, t66: f64) -> (f64, f64, f64, f64) {
    let t9709 = t9707 * t550 * t816;
    let t9711 = 0.12846167376791569079e-2_f64 * t1379 * t9709;
    let t9712 = t2689 * t3952;
    let t9718 = t9646 * t547;
    let t9720 = 1.0_f64 / t66 / t2236;
    (t9711, t9712, t9718, t9720)
}
