//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1910/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1910(t1873: f64, t94519: f64, t26004: f64, t5690: f64, t13951: f64, t2018: f64, t807: f64, t25240: f64, t3964: f64, t5617: f64, t543: f64, t97870: f64) -> (f64, f64, f64, f64, f64) {
    let t98260 = t94519 * t1873;
    let t98269 = t26004 * t5690;
    let t98281 = t807 * t2018 * t13951;
    let t98285 = t3964 * t25240 * t5617;
    let t98299 = t97870 * t543;
    (t98260, t98269, t98281, t98285, t98299)
}
