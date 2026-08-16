//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1066/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1066(t1794: f64, t6587: f64, t1250: f64, t3720: f64, t1715: f64, t20809: f64, t1042: f64, t5192: f64, t6548: f64, t12552: f64, t24375: f64, t12555: f64) -> (f64, f64, f64, f64, f64) {
    let t24751 = t6587 * t1794;
    let t24752 = t24751 * t1250;
    let t24753 = t3720 * t24752;
    let t24758 = t20809 * t1715;
    let t24759 = t1042 * t24758;
    let t24763 = 0.35089341735807877242e1_f64 * t5192 * t6548;
    let t24764 = t12552 * t24375;
    let t24765 = t24764 * t12555;
    (t24751, t24753, t24759, t24763, t24765)
}
