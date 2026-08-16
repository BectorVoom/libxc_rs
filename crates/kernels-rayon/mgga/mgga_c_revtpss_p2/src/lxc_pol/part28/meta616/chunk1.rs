//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2153/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2153(t212: f64, t27265: f64, t689: f64, t780: f64, t1558: f64, t25391: f64, t25392: f64, t25394: f64, t92841: f64, t92844: f64, t92847: f64, t92856: f64, t92858: f64, t92861: f64, t98803: f64, t98806: f64, t98811: f64, t98814: f64, t98817: f64, t98825: f64) -> f64 {
    let t98830 = 0.10975748638225852664e-1_f64 * t689 * t212 * t27265 * t780;
    let t98831 = -0.51405703062096148812e-1_f64 * t92841 + 0.28912093960683998208e-1_f64 * t92844 - t98803 + t98806 + t98811 - t98814 - t98817 + 0.9757440539382783019e-2_f64 * t92847 + 0.54878743191129263322e-2_f64 * t92856 - 0.14634331517634470219e-1_f64 * t92858 - 0.17347256376410398924e1_f64 * t25391 * t25392 * t1558 * t25394 + 0.17135234354032049604e-1_f64 * t98825 + t92861 - t98830;
    t98831
}
