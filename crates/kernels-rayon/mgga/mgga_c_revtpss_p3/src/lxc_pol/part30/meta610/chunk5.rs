//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2089/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2089(t13730: f64, t2023: f64, t2782: f64, t10073: f64, t25938: f64, t27836: f64, t14079: f64, t26054: f64, t7289: f64, t97925: f64, t1882: f64, t25921: f64, t25930: f64, t25931: f64, t25933: f64, t26036: f64, t27853: f64, t27972: f64, t7917: f64, t94716: f64, t94779: f64, t94784: f64, t94799: f64, t94803: f64, t94807: f64, t94811: f64, t94813: f64) -> f64 {
    let t98001 = 0.21951497276451705328e-1_f64 * t2782 * t2023 * t13730;
    let t98003 = t10073 * t27836 * t25938;
    let t98010 = 0.19514881078765566038e-1_f64 * t26054 * t14079;
    let t98011 = t7289 * t97925;
    let t98022 = -0.68540937416128198416e-1_f64 * t94779 - 0.17347256376410398924e1_f64 * t25930 * t94716 * t27972 - t98001 - 0.24093411633903331839e-3_f64 * t98003 + 0.8673628188205199462e0_f64 * t25921 * t27853 - 0.4336814094102599731e0_f64 * t7917 * t26036 + t94784 - t98010 + 0.17135234354032049604e-1_f64 * t98011 - 0.17347256376410398924e1_f64 * t25930 * t25931 * t1882 * t25933 - 0.9757440539382783019e-2_f64 * t94799 + 0.25702851531048074406e-1_f64 * t94803 + 0.34270468708064099208e-2_f64 * t94807 + 0.72280234901709995518e-2_f64 * t94811 + 0.51405703062096148812e-1_f64 * t94813;
    t98022
}
