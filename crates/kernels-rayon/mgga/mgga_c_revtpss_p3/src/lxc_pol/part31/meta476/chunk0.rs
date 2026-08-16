//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1746/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1746(t25227: f64, t2664: f64, t2661: f64, t2670: f64, t7033: f64, t2482: f64, t27: f64, t7043: f64) -> (f64, f64, f64, f64) {
    let t25228 = t25227 * t2664;
    let t25229 = t2661 * t25228;
    let t25231 = t7033 * t2670;
    let t25232 = 0.27104001498285508387e-3_f64 * t25231;
    let t25234 = t2482 * t7043 * t27;
    (t25228, t25229, t25232, t25234)
}
