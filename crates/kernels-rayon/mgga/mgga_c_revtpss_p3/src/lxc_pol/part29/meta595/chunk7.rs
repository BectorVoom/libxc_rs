//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2002/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2002(t786: f64, t7998: f64, t867: f64, t2467: f64, t1580: f64, t26446: f64, t689: f64, t28368: f64, t93321: f64, t93374: f64, t25317: f64, t26511: f64, t26550: f64, t26551: f64, t26568: f64, t26573: f64, t27199: f64, t27353: f64, t2771: f64, t28400: f64, t51698: f64, t7067: f64, t7070: f64, t7997: f64, t93349: f64, t95740: f64, t95744: f64, t95747: f64, t99191: f64, t99277: f64) -> f64 {
    let t103067 = t786 * t7998 * t867;
    let t103069 = 0.19514881078765566038e-1_f64 * t103067 * t2467;
    let t103072 = 0.10975748638225852664e-1_f64 * t689 * t26446 * t1580;
    let t103086 = 0.14456046980341999104e-1_f64 * t93321 * t28368;
    let t103088 = 0.25702851531048074406e-1_f64 * t93374 * t28368;
    let t103100 = -t103069 + t103072 - 0.26020884564615598386e1_f64 * t7070 * t25317 * t7997 * t2771 - 0.48186823267806663678e-3_f64 * t95740 - 0.19514881078765566038e-1_f64 * t95744 + 0.26020884564615598386e1_f64 * t93349 * t26550 * t99277 - 0.8673628188205199462e0_f64 * t27199 * t26511 + 0.45699670022203476294e-2_f64 * t95747 - t103086 + t103088 + 0.4336814094102599731e0_f64 * t27353 * t26550 * t51698 + 0.8673628188205199462e0_f64 * t27199 * t26568 + 0.4336814094102599731e0_f64 * t27199 * t26573 - 0.8673628188205199462e0_f64 * t7067 * t28400 - 0.17347256376410398924e1_f64 * t99191 * t26551;
    t103100
}
