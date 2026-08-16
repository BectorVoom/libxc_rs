//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 836/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk836(t12351: f64, t1178: f64, t3519: f64, t439: f64, t3522: f64, t447: f64, t3800: f64, t498: f64, t12295: f64, t1207: f64, t456: f64, t487: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12543 = 0.36793333333333333333e0_f64 * t12351;
    let t12552 = 1.0_f64 / t3519 / t1178;
    let t12553 = t439 * t12552;
    let t12555 = 1.0_f64 / t3522 / t447;
    let t12587 = 1.0_f64 / t3800 / t498;
    let t12610 = 0.46096296296296296297e-1_f64 * t12295;
    let t12625 = t1207 * t1207;
    let t12626 = 1.0_f64 / t12625;
    let t12627 = t456 * t12626;
    let t12628 = t12627 * t487;
    (t12543, t12552, t12553, t12555, t12587, t12610, t12627, t12628)
}
