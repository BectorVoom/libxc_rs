//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1253/1507 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1253(t4533: f64, t886: f64, t2770: f64, t1579: f64, t2828: f64, t10989: f64, t10992: f64, t10998: f64, t11000: f64, t11004: f64, t11013: f64, t11017: f64, t11019: f64, t11022: f64, t2765: f64, t4487: f64, t4534: f64, t865: f64) -> f64 {
    let t15029 = t4533 * t886;
    let t15030 = t2770 * t15029;
    let t15038 = t2770 * t1579 * t2828;
    let t15044 = 0.54878743191129263322e-2_f64 * t10989 + 0.10975748638225852664e-1_f64 * t10992 + 0.19514881078765566038e-1_f64 * t10998 - 0.14634331517634470219e-1_f64 * t11000 + 0.13009920719177044025e-2_f64 * t11004 + 0.26341796731742046394e1_f64 * t865 * t15030 - 0.13170898365871023197e1_f64 * t2765 * t4534 + 0.26341796731742046394e1_f64 * t2765 * t4487 + 0.13170898365871023197e1_f64 * t865 * t15038 - 0.2601984143835408805e-1_f64 * t11013 + t11017 + 0.23131639038696784278e-2_f64 * t11019 + 0.9757440539382783019e-2_f64 * t11022;
    t15044
}
