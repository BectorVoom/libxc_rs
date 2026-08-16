//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1047/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1047(t2408: f64, t33: f64, t1113: f64, t890: f64, t2832: f64, t4135: f64, t4147: f64, t112: f64, t239: f64, t624: f64, t655: f64, t665: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25778 = t33 * t2408;
    let t25781 = t1113 * t890;
    let t25784 = t33 * t2832;
    let t25802 = t4147 * t4135;
    let t25821 = t239 * t112;
    let t25823 = t624 * t655;
    let t25824 = t25823 * t665;
    (t25778, t25781, t25784, t25802, t25821, t25823, t25824)
}
