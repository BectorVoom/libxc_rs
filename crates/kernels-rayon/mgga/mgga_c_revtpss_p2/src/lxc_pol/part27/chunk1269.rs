//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1269/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1269(t2435: f64, t26061: f64, t1385: f64, t7274: f64, t1398: f64, t4131: f64, t543: f64, t2453: f64, t26053: f64, t9676: f64, t4078: f64, t689: f64, t7242: f64) -> (f64, f64, f64, f64, f64) {
    let t94714 = t2435 * t26061;
    let t94716 = t1385 * t7274;
    let t94721 = t4131 * t1398 * t543;
    let t94725 = t2453 * t26053;
    let t94726 = t94725 * t9676;
    let t94729 = t689 * t7242 * t4078;
    (t94714, t94716, t94721, t94726, t94729)
}
