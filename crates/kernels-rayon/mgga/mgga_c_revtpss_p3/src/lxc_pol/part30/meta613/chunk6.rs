//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2113/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2113(t5775: f64, t689: f64, t7242: f64, t25898: f64, t98040: f64, t25901: f64, t25878: f64, t98356: f64, t27989: f64, t94921: f64, t13747: f64, t1882: f64, t25930: f64, t27980: f64, t7279: f64, t94898: f64, t94902: f64, t94904: f64, t98358: f64, t98360: f64, t98362: f64, t98368: f64, t98372: f64, t98376: f64) -> f64 {
    let t98379 = 0.10975748638225852664e-1_f64 * t689 * t7242 * t5775;
    let t98380 = t98040 * t25898;
    let t98382 = 0.25702851531048074406e-1_f64 * t98380 * t25901;
    let t98384 = 0.51405703062096148812e-1_f64 * t25878 * t98356;
    let t98387 = 0.14456046980341999104e-1_f64 * t94921 * t27989;
    let t98388 = -t98358 - t98360 + 0.34694512752820797848e1_f64 * t25930 * t27980 * t1882 * t98362 + 0.54878743191129263322e-2_f64 * t94898 - t98368 + 0.19514881078765566038e-1_f64 * t94902 + 0.26341796731742046394e1_f64 * t7279 * t13747 - 0.13009920719177044025e-1_f64 * t98372 + t98376 + t98379 + t98382 + t98384 + 0.14456046980341999104e-1_f64 * t94904 - t98387;
    t98388
}
