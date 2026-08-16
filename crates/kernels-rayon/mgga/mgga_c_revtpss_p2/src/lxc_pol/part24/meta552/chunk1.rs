//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1643/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1643(t11506: f64, t3014: f64, t88008: f64, t981: f64, t1610: f64, t78097: f64, t19056: f64, t6142: f64, t6145: f64, t64336: f64, t23547: f64, t4590: f64) -> (f64, f64, f64, f64, f64) {
    let t88046 = 0.6233709278045326953e3_f64 * t981 * t11506 * t88008 * t3014;
    let t88048 = 4.0_f64 * t78097 * t1610;
    let t88050 = 6.0_f64 * t19056 * t6142;
    let t88052 = 0.96491876992155210402e2_f64 * t64336 * t6145;
    let t88054 = 4.0_f64 * t4590 * t23547;
    (t88046, t88048, t88050, t88052, t88054)
}
