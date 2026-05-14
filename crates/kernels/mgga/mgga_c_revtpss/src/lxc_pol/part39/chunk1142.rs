//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1142/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1142<F: Float>(t15029: F, t2770: F, t1579: F, t2828: F, t10989: F, t10992: F, t10998: F, t11000: F, t11004: F, t11013: F, t11017: F, t11019: F, t11022: F, t2765: F, t4487: F, t4534: F, t865: F) -> (F,) {
    let t15030 = t2770 * t15029;
    let t15038 = t2770 * t1579 * t2828;
    let t15044 = 0.54878743191129263322e-2 * t10989 + 0.10975748638225852664e-1 * t10992 + 0.19514881078765566038e-1 * t10998 - 0.14634331517634470219e-1 * t11000 + 0.13009920719177044025e-2 * t11004 + 0.26341796731742046394e1 * t865 * t15030 - 0.13170898365871023197e1 * t2765 * t4534 + 0.26341796731742046394e1 * t2765 * t4487 + 0.13170898365871023197e1 * t865 * t15038 - 0.2601984143835408805e-1 * t11013 + t11017 + 0.23131639038696784278e-2 * t11019 + 0.9757440539382783019e-2 * t11022;
    (t15044,)
}
