//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2765/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2765(t4010: f64, t6816: f64, t1353: f64, t13767: f64, t2661: f64, t22027: f64, t9775: f64, t22252: f64, t3992: f64, t543: f64, t550: f64, t22263: f64) -> (f64, f64, f64, f64, f64) {
    let t74012 = t4010 * t6816;
    let t74015 = t2661 * t13767 * t74012 * t1353;
    let t74017 = t9775 * t22027;
    let t74022 = t2661 * t3992 * t550 * t22252 * t543;
    let t74024 = t9775 * t22263;
    (t74012, t74015, t74017, t74022, t74024)
}
