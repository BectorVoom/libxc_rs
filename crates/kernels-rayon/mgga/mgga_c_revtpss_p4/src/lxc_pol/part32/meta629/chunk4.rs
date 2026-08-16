//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2023/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2023(t103184: f64, t103196: f64, t103216: f64, t103219: f64, t103220: f64, t110444: f64, t110453: f64, t110459: f64, t14587: f64, t1579: f64, t213: f64, t225: f64, t25317: f64, t25391: f64, t257: f64, t27199: f64, t28418: f64, t28425: f64, t30356: f64, t7070: f64, t886: f64, t95807: f64, t95808: f64, t95811: f64, t95813: f64) -> f64 {
    let t110466 = -t103184 + 0.17347256376410398924e1_f64 * t27199 * t28418 + 0.65854491829355115987e0_f64 * t213 * t110444 * t225 * t257 + 0.34694512752820797848e1_f64 * t25391 * t28425 * t1579 * t14587 - 0.25702851531048074406e-1_f64 * t110453 + t103196 + t95807 - 0.22849835011101738147e-2_f64 * t95808 + t103216 + 0.24093411633903331839e-3_f64 * t95811 - t103219 + 0.26019841438354088051e-1_f64 * t103220 + 0.9757440539382783019e-2_f64 * t110459 - 0.22849835011101738147e-2_f64 * t95813 - 0.26020884564615598386e1_f64 * t7070 * t25317 * t30356 * t886;
    t110466
}
