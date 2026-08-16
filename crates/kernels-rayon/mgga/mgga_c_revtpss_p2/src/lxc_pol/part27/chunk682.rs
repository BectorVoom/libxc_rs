//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 682/1333 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk682(t3014: f64, t972: f64, t3093: f64, t357: f64, t1065: f64, t2857: f64, t2852: f64, t3181: f64, t1062: f64, t3204: f64, t3147: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4733 = t3014 * t972;
    let t4786 = t3093 * t357;
    let t4801 = t1065 * t2857;
    let t4806 = t3181 * t2852;
    let t4837 = t3204 * t1062;
    let t4890 = t3147 * t72;
    (t4733, t4786, t4801, t4806, t4837, t4890)
}
