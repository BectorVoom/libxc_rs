//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 933/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk933(t555: f64, t6843: f64, t550: f64, t543: f64, t3992: f64, t2661: f64, t6861: f64, t4003: f64, t9934: f64, t3989: f64, t6856: f64, t3957: f64, t6884: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t22009 = t555 * t6843;
    let t22020 = t550 * t6843;
    let t22021 = t22020 * t543;
    let t22022 = t3992 * t22021;
    let t22023 = t2661 * t22022;
    let t22025 = t550 * t6861;
    let t22026 = t22025 * t4003;
    let t22027 = t9934 * t22026;
    let t22028 = t2661 * t22027;
    let t22030 = t3989 * t6856;
    let t22038 = t3957 * t6884;
    (t22009, t22021, t22023, t22025, t22026, t22028, t22030, t22038)
}
