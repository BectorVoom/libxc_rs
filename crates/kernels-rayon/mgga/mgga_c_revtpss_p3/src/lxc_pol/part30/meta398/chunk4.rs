//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1495/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1495(t14567: f64, t786: f64, t2801: f64, t10533: f64, t10539: f64, t10543: f64, t10548: f64, t10645: f64, t10647: f64, t10651: f64, t10655: f64, t14546: f64, t14547: f64, t14558: f64, t14564: f64, t2646: f64, t2724: f64, t2754: f64, t4494: f64, t4504: f64, t4514: f64, t4526: f64, t820: f64) -> f64 {
    let t14568 = t786 * t14567;
    let t14570 = 0.19514881078765566038e-1_f64 * t14568 * t2801;
    let t14572 = 0.19514881078765566038e-1_f64 * t10533 - 0.23131639038696784278e-2_f64 * t10539 - 0.19514881078765566038e-1_f64 * t10543 - 0.9757440539382783019e-2_f64 * t10548 - 0.39512695097613069591e1_f64 * t14546 * t4494 * t14547 + 0.39512695097613069591e1_f64 * t4504 * t4494 * t2724 - 0.65854491829355115987e0_f64 * t4514 * t4494 * t2646 - 0.65049603595885220126e-3_f64 * t14558 - 0.65854491829355115987e0_f64 * t820 * t4526 * t2754 + 0.13009920719177044025e-1_f64 * t14564 - t10645 - 0.2601984143835408805e-1_f64 * t10647 + t10651 - t14570 - 0.10975748638225852664e-1_f64 * t10655;
    t14572
}
