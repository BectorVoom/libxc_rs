//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1633/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1633(t1357: f64, t6919: f64, t689: f64, t1444: f64, t6918: f64, t4076: f64, t14081: f64, t14084: f64, t14087: f64, t1424: f64, t14299: f64, t1904: f64, t22395: f64, t22400: f64, t22405: f64, t22407: f64, t9677: f64, t9687: f64, t9691: f64) -> (f64, f64) {
    let t22409 = t1357 * t6919;
    let t22410 = t689 * t22409;
    let t22414 = t6918 * t1444;
    let t22415 = t4076 * t22414;
    let t22418 = 0.26341796731742046394e1_f64 * t1424 * t22395 - 0.9757440539382783019e-2_f64 * t22400 - 0.11565819519348392139e-2_f64 * t9677 + 0.13009920719177044025e-1_f64 * t9687 + 0.9757440539382783019e-2_f64 * t22405 - t14081 + t14084 - 0.19514881078765566037e-1_f64 * t22407 + 0.54878743191129263322e-2_f64 * t22410 + t14087 - t9691 - 0.13170898365871023197e1_f64 * t14299 * t1904 + 0.13170898365871023197e1_f64 * t1424 * t22415;
    (t22415, t22418)
}
