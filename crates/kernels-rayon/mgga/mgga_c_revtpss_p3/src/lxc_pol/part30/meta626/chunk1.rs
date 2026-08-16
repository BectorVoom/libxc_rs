//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2169/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2169(t1579: f64, t2722: f64, t231: f64, t27266: f64, t686: f64, t72: f64, t7058: f64, t1959: f64, t25391: f64, t25392: f64, t27353: f64, t51436: f64, t51698: f64, t7079: f64, t93242: f64, t93252: f64, t93262: f64, t93272: f64, t93273: f64, t99297: f64, t99300: f64, t99303: f64, t99307: f64, t99309: f64, t99313: f64) -> (f64, f64, f64) {
    let t99315 = t1579 * t2722;
    let t99316 = t99315 * t231;
    let t99321 = t27266 * t72 * t686;
    let t99323 = 0.14456046980341999104e-1_f64 * t7058 * t99321;
    let t99332 = -0.24093411633903331839e-3_f64 * t99297 + 0.48186823267806663678e-3_f64 * t93242 - 0.4336814094102599731e0_f64 * t99300 * t1959 + 0.8673628188205199462e0_f64 * t99303 * t7079 + 0.23131639038696784278e-2_f64 * t93252 - 0.13009920719177044025e-1_f64 * t99307 - 0.8673628188205199462e0_f64 * t25391 * t25392 * t99309 - 0.11565819519348392139e-2_f64 * t99313 - 0.8673628188205199462e0_f64 * t25391 * t25392 * t99316 + t99323 + 0.19514881078765566038e-1_f64 * t93262 + 0.8673628188205199462e0_f64 * t27353 * t25392 * t51436 + 0.4336814094102599731e0_f64 * t27353 * t25392 * t51698 + t93272 + 0.2601984143835408805e-1_f64 * t93273;
    (t99315, t99321, t99332)
}
