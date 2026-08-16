//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 636/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk636(t2242: f64, t38: f64, t644: f64, t84: f64, t77: f64, t603: f64, t607: f64, t624: f64, t640: f64, t76: f64, t1937: f64, t2322: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6954 = t2242 * t38;
    let t6959 = t84 * t644;
    let t6960 = t77 * t6959;
    let t6963 = t603 * t607;
    let t6971 = 8.0_f64 / 3.0_f64 * t624;
    let t6977 = t76 * t640;
    let t6990 = 2.0_f64 * t2322 * t1937;
    (t6954, t6960, t6963, t6971, t6977, t6990)
}
