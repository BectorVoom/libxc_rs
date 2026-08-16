//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1074/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1074(t12485: f64, t439: f64, t1175: f64, t3495: f64, t1156: f64, t3451: f64, t12295: f64, t12351: f64, t1178: f64, t3519: f64, t3522: f64, t447: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12486 = t439 * t12485;
    let t12491 = t1175 * t3495;
    let t12511 = t1156 * t3451;
    let t12542 = 0.93932222222222222223e0_f64 * t12295;
    let t12543 = 0.36793333333333333333e0_f64 * t12351;
    let t12552 = 1.0_f64 / t3519 / t1178;
    let t12553 = t439 * t12552;
    let t12555 = 1.0_f64 / t3522 / t447;
    (t12486, t12491, t12511, t12542, t12543, t12552, t12553, t12555)
}
