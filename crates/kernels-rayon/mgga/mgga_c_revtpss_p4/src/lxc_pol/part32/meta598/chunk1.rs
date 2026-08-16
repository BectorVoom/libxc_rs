//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1933/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1933(t1513: f64, t4287: f64, t25826: f64, t25823: f64, t5915: f64, t665: f64, t21876: f64, t6998: f64, t28166: f64, t7897: f64, t5824: f64, t775: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t105875 = t1513 * t4287;
    let t105876 = t25826 * t105875;
    let t105878 = t25823 * t5915;
    let t105880 = t5915 * t665;
    let t105881 = t25826 * t105880;
    let t105883 = t6998 * t21876;
    let t105892 = t7897 * t28166;
    let t105898 = t5824 * t775;
    (t105876, t105878, t105881, t105883, t105892, t105898)
}
