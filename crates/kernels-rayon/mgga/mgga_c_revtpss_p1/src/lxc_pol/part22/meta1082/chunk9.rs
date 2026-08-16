//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3910/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3910(t10073: f64, t22361: f64, t10069: f64, t22373: f64, t10139: f64, t136: f64, t2457: f64, t6874: f64, t1399: f64, t14255: f64, t46536: f64, t46542: f64, t49198: f64, t49200: f64, t49203: f64, t49208: f64, t49210: f64, t5659: f64, t5675: f64, t5745: f64, t5755: f64, t74965: f64, t74982: f64, t820: f64) -> f64 {
    let t75113 = t10073 * t22361;
    let t75119 = t10069 * t22373;
    let t75123 = t10139 * t6874 * t136 * t2457;
    let t75125 = -0.13170898365871023197e1_f64 * t5755 * t74982 * t1399 - 0.26341796731742046394e1_f64 * t820 * t14255 * t5659 + 0.52039682876708176102e-1_f64 * t49198 - 0.19514881078765566038e-1_f64 * t49200 - 0.60712963356159538786e-1_f64 * t49203 + 0.21951497276451705328e-1_f64 * t49208 - 0.520396828767081761e-2_f64 * t49210 - 0.73171657588172351096e-2_f64 * t46536 - 0.13009920719177044025e-2_f64 * t75113 + 0.26341796731742046394e1_f64 * t5745 * t74965 * t5675 - 0.73171657588172351096e-2_f64 * t46542 - 0.73171657588172351096e-2_f64 * t75119 - 0.11565819519348392139e-2_f64 * t75123;
    t75125
}
