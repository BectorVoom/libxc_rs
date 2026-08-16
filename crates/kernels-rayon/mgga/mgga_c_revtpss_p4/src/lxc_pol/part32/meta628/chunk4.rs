//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2013/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2013(t103067: f64, t4481: f64, t103140: f64, t103142: f64, t103156: f64, t103158: f64, t103161: f64, t106275: f64, t26550: f64, t27353: f64, t62695: f64, t7415: f64, t95774: f64, t95779: f64, t95783: f64, t95786: f64, t95794: f64, t95796: f64) -> f64 {
    let t110355 = t103067 * t4481;
    let t110365 = t103140 + t103142 + 0.8673628188205199462e0_f64 * t106275 * t7415 - 0.11565819519348392139e-2_f64 * t95774 + 0.13009920719177044025e-1_f64 * t95779 - 0.24093411633903331839e-3_f64 * t95783 - 0.19514881078765566037e-1_f64 * t110355 + 0.4336814094102599731e0_f64 * t27353 * t26550 * t62695 - 0.17135234354032049604e-1_f64 * t95786 + t103156 + 0.13009920719177044025e-2_f64 * t103158 + 0.23131639038696784278e-2_f64 * t103161 + 0.17135234354032049604e-2_f64 * t95794 + 0.96373646535613327357e-2_f64 * t95796;
    t110365
}
