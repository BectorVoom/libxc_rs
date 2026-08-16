//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2745/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2745(t17708: f64, t59498: f64, t12916: f64, t21041: f64, t3718: f64, t21165: f64, t12809: f64, t20796: f64, t13045: f64, t5284: f64, t5245: f64, t5457: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t72011 = t59498 * t17708;
    let t72017 = t3718 * t12916 * t21041;
    let t72064 = t3718 * t12916 * t21165;
    let t72071 = t12809 * t12916 * t20796;
    let t72086 = t13045 * t5284;
    let t72143 = t5457 * t5245;
    (t72011, t72017, t72064, t72071, t72086, t72143)
}
