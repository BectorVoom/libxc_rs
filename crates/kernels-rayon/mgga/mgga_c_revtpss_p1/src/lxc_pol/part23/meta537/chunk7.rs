//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2082/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2082(t22020: f64, t543: f64, t3992: f64, t2661: f64, t550: f64, t6861: f64) -> (f64, f64, f64, f64) {
    let t22021 = t22020 * t543;
    let t22022 = t3992 * t22021;
    let t22023 = t2661 * t22022;
    let t22025 = t550 * t6861;
    (t22021, t22022, t22023, t22025)
}
