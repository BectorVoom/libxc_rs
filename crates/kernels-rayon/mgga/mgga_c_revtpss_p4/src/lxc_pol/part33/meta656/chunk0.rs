//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2109/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2109(t28056: f64, t7732: f64, t5891: f64, t94978: f64, t665: f64, t94982: f64, t1513: f64, t4287: f64, t25826: f64, t25823: f64, t5915: f64, t21876: f64, t6998: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t105863 = 4.0_f64 * t7732 * t28056;
    let t105870 = t94978 * t5891;
    let t105872 = t5891 * t665;
    let t105873 = t94982 * t105872;
    let t105875 = t1513 * t4287;
    let t105876 = t25826 * t105875;
    let t105878 = t25823 * t5915;
    let t105880 = t5915 * t665;
    let t105881 = t25826 * t105880;
    let t105883 = t6998 * t21876;
    (t105863, t105870, t105873, t105876, t105878, t105881, t105883)
}
