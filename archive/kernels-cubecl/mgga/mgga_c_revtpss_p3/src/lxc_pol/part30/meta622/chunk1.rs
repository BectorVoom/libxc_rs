//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2138/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2138<F: Float>(t212: F, t27265: F, t689: F, t780: F, t1558: F, t25391: F, t25392: F, t25394: F, t92841: F, t92844: F, t92847: F, t92856: F, t92858: F, t92861: F, t98803: F, t98806: F, t98811: F, t98814: F, t98817: F, t98825: F) -> F {
    let t98830 = F::cast_from(0.10975748638225852664e-1_f64) * t689 * t212 * t27265 * t780;
    let t98831 = -F::cast_from(0.51405703062096148812e-1_f64) * t92841 + F::cast_from(0.28912093960683998208e-1_f64) * t92844 - t98803 + t98806 + t98811 - t98814 - t98817 + F::cast_from(0.9757440539382783019e-2_f64) * t92847 + F::cast_from(0.54878743191129263322e-2_f64) * t92856 - F::cast_from(0.14634331517634470219e-1_f64) * t92858 - F::cast_from(0.17347256376410398924e1_f64) * t25391 * t25392 * t1558 * t25394 + F::cast_from(0.17135234354032049604e-1_f64) * t98825 + t92861 - t98830;
    t98831
}
