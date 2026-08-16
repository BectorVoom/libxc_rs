//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2176/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2176(t25331: f64, t27213: f64, t93190: f64, t99211: f64, t25374: f64, t98848: f64, t25378: f64, t99403: f64, t231: f64, t2645: f64, t27265: f64, t7070: f64, t7076: f64, t7759: f64, t836: f64, t93326: f64, t93331: f64, t93334: f64, t93335: f64, t93337: f64, t93339: f64, t93343: f64, t93346: f64, t93365: f64) -> f64 {
    let t99456 = t27213 * t25331;
    let t99460 = t93190 * t99211;
    let t99463 = t98848 * t25374;
    let t99465 = 0.51405703062096148812e-1_f64 * t99463 * t25378;
    let t99466 = t99403 * t25374;
    let t99468 = 0.28912093960683998208e-1_f64 * t99466 * t25378;
    let t99469 = 0.8673628188205199462e0_f64 * t7070 * t7076 * t27265 * t836 * t231 + 0.4336814094102599731e0_f64 * t7070 * t7076 * t7759 * t2645 * t231 - 0.14456046980341999104e-1_f64 * t93326 - 0.28912093960683998208e-1_f64 * t93331 - t93334 - 0.34270468708064099208e-1_f64 * t93335 - 0.72280234901709995518e-2_f64 * t93337 - 0.68540937416128198416e-1_f64 * t93339 - 0.96373646535613327357e-2_f64 * t99456 + 0.51405703062096148812e-1_f64 * t93343 - 0.9757440539382783019e-2_f64 * t93346 + 0.45699670022203476294e-2_f64 * t99460 - 0.28912093960683998208e-1_f64 * t93365 + t99465 - t99468;
    t99469
}
