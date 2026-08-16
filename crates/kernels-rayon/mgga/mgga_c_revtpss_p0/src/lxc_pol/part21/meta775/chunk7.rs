//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2765/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2765(t4398: f64, t9323: f64, t39989: f64, t40137: f64, t40141: f64, t50065: f64, t50070: f64, t50085: f64, t50091: f64, t50093: f64, t50095: f64, t50096: f64, t50098: f64, t50100: f64, t50101: f64, t50106: f64, t50114: f64, t50115: f64) -> (f64, f64) {
    let t50852 = t4398 * t9323;
    let t50853 = 0.51947577317044391277e2_f64 * t50852;
    let t50854 = -t50065 - t40137 + t50070 + t50085 + t50091 + t50093 + t50095 + t40141 + t50096 + t50098 + t50100 + t50101 + t50106 - t39989 + t50114 + t50115 - t50853;
    (t50853, t50854)
}
