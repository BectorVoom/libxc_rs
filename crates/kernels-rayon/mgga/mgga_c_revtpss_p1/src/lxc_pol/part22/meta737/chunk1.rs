//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2798/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2798(t10887: f64, t40721: f64, t136: f64, t2475: f64, t220: f64, t2482: f64, t2668: f64, t823: f64) -> (f64, f64, f64, f64) {
    let t40722 = t40721 * t10887;
    let t40724 = t2475 * t136;
    let t40725 = t40724 * t220;
    let t40731 = t2482 * t823 * t2668;
    (t40722, t40724, t40725, t40731)
}
