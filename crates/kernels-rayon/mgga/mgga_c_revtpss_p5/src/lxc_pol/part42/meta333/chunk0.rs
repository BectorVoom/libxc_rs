//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1129/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1129(t240: f64, t849: f64, t14648: f64, t775: f64, t2661: f64, t2652: f64, t4345: f64, t10716: f64, t4349: f64, t2689: f64, t4372: f64, t4354: f64, t9775: f64) -> (f64, f64, f64, f64, f64) {
    let t14832 = t849 * t240;
    let t14833 = t14648 * t775;
    let t14834 = t14832 * t14833;
    let t14836 = 0.28582678745379824648e-3_f64 * t2661 * t14834;
    let t14837 = t2652 * t4345;
    let t14839 = t10716 * t4349;
    let t14846 = t2689 * t4372;
    let t14850 = t9775 * t4354;
    (t14836, t14837, t14839, t14846, t14850)
}
