//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 809/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk809(t243: f64, t816: f64, t9707: f64, t813: f64, t247: f64, t9949: f64, t237: f64, t236: f64, t9646: f64, t9721: f64, t268: f64, t207: f64, t242: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10671 = t9707 * t243 * t816;
    let t10673 = 0.12846167376791569079e-2_f64 * t813 * t10671;
    let t10685 = t9949 * t243 * t247;
    let t10687 = 0.37792653007779990369e-1_f64 * t237 * t10685;
    let t10688 = t9646 * t236;
    let t10689 = t9721 * t243;
    let t10690 = t10689 * t268;
    let t10692 = 0.20082057720118594944e-6_f64 * t10688 * t10690;
    let t10696 = 1.0_f64 / t242 / t207;
    (t10671, t10673, t10685, t10687, t10690, t10692, t10696)
}
