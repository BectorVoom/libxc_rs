//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2023/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2023<F: Float>(t103184: F, t103196: F, t103216: F, t103219: F, t103220: F, t110444: F, t110453: F, t110459: F, t14587: F, t1579: F, t213: F, t225: F, t25317: F, t25391: F, t257: F, t27199: F, t28418: F, t28425: F, t30356: F, t7070: F, t886: F, t95807: F, t95808: F, t95811: F, t95813: F) -> F {
    let t110466 = -t103184 + F::cast_from(0.17347256376410398924e1_f64) * t27199 * t28418 + F::cast_from(0.65854491829355115987e0_f64) * t213 * t110444 * t225 * t257 + F::cast_from(0.34694512752820797848e1_f64) * t25391 * t28425 * t1579 * t14587 - F::cast_from(0.25702851531048074406e-1_f64) * t110453 + t103196 + t95807 - F::cast_from(0.22849835011101738147e-2_f64) * t95808 + t103216 + F::cast_from(0.24093411633903331839e-3_f64) * t95811 - t103219 + F::cast_from(0.26019841438354088051e-1_f64) * t103220 + F::cast_from(0.9757440539382783019e-2_f64) * t110459 - F::cast_from(0.22849835011101738147e-2_f64) * t95813 - F::cast_from(0.26020884564615598386e1_f64) * t7070 * t25317 * t30356 * t886;
    t110466
}
