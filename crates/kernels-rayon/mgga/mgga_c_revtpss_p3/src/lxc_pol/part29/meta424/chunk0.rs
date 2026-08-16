//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1562/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1562(t141: f64, t16886: f64, t1145: f64, t16733: f64, t5098: f64, t698: f64, t16725: f64, t3417: f64, t16729: f64, t16720: f64, t16738: f64, t12254: f64, t16715: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16887 = t141 * t16886;
    let t16889 = t1145 * t16733;
    let t16890 = t141 * t16889;
    let t16892 = t698 * t5098;
    let t16893 = 0.21908444444444444444e0_f64 * t16892;
    let t16894 = t3417 * t16725;
    let t16895 = t141 * t16894;
    let t16897 = t3417 * t16729;
    let t16898 = t141 * t16897;
    let t16900 = t3417 * t16720;
    let t16901 = t141 * t16900;
    let t16903 = t1145 * t16738;
    let t16904 = t141 * t16903;
    let t16907 = t12254 * t16715;
    (t16887, t16890, t16892, t16893, t16895, t16898, t16901, t16904, t16907)
}
