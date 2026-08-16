//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 66/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk66(t128: f64, t131: f64, t134: f64, t141: f64) -> (f64, f64, f64, f64) {
    let t169 = 0.705945e1_f64 * t131 + 0.1549425e1_f64 * t128 + 0.420775e0_f64 * t134 + 0.1562925e0_f64 * t141;
    let t172 = 1.0_f64 + 0.32163958997385070134e2_f64 / t169;
    let t173 = f64::ln(t172);
    let t177 = 1.0_f64 + 0.278125e-1_f64 * t128;
    (t169, t172, t173, t177)
}
