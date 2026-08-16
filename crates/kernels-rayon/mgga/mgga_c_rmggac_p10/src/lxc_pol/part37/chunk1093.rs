//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 1093/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk1093(t75362: f64, t75364: f64, t75356: f64, t75367: f64, t75369: f64, t75371: f64, t75378: f64, t75380: f64, t75383: f64, t78148: f64, t78150: f64, t78156: f64, t78157: f64, t78161: f64, t78162: f64, t78163: f64) -> f64 {
    let t80358 = 0.2419210303588817044e-2_f64 * t75362;
    let t80359 = 0.33868944250243438616e-2_f64 * t75364;
    let t80366 = t78148 - 0.50803416375365157924e-2_f64 * t75356 + t78150 + t80358 - t80359 - 0.68186654135613354324e-2_f64 * t75367 - 0.68186654135613354324e-2_f64 * t75369 + 0.13637330827122670865e-1_f64 * t75371 + t78156 + t78157 + 0.2727466165424534173e-1_f64 * t75378 + 0.2727466165424534173e-1_f64 * t75380 - 0.68186654135613354325e-1_f64 * t75383 + t78161 - t78162 - t78163;
    t80366
}
