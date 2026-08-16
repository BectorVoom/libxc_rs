//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1763/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1763(t10652: f64, t2723: f64, t4503: f64, t2782: f64, t2760: f64, t822: f64, t2718: f64, t860: f64, t243: f64, t816: f64, t9707: f64, t813: f64) -> (f64, f64, f64, f64, f64) {
    let t10654 = t4503 * t10652 * t2723;
    let t10655 = t2782 * t10654;
    let t10657 = t822 * t2760;
    let t10661 = t2718 * t860;
    let t10671 = t9707 * t243 * t816;
    let t10673 = 0.12846167376791569079e-2_f64 * t813 * t10671;
    (t10654, t10655, t10657, t10661, t10673)
}
