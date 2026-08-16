//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1216/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1216(t103114: f64, t103122: f64, t103130: f64, t103158: f64, t103161: f64, t110340: f64, t110344: f64, t110346: f64, t110355: f64, t1558: f64, t1579: f64, t231: f64, t25317: f64, t27199: f64, t28394: f64, t30337: f64, t30379: f64, t30392: f64, t6049: f64, t6071: f64, t7070: f64, t7071: f64, t7076: f64, t8006: f64, t99191: f64) -> f64 {
    let t115551 = 0.26020884564615598386e1_f64 * t7070 * t7071 * t30379 * t1579 + 0.13010442282307799193e1_f64 * t7070 * t7076 * t30379 * t1558 * t231 - 0.78062653693846795158e1_f64 * t7070 * t25317 * t8006 * t6071 - 0.26020884564615598386e1_f64 * t27199 * t30392 + 0.39512695097613069591e1_f64 * t28394 * t6049 - 0.28912093960683998208e-1_f64 * t103114 + 0.21684070470512998656e-1_f64 * t110340 + 0.68549505033305214441e-2_f64 * t103122 + 0.72280234901709995519e-3_f64 * t103130 - 0.86736281882051994623e-1_f64 * t110344 - 0.38554277296572111609e-1_f64 * t110346 - 0.58544643236296698113e-1_f64 * t110355 - 0.52041769129231196772e1_f64 * t99191 * t30337 + 0.19514881078765566038e-2_f64 * t103158 + 0.34697458558045176417e-2_f64 * t103161;
    t115551
}
