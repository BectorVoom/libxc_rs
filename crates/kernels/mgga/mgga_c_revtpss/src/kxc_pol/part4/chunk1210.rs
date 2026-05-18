//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1210/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1210<F: Float>(t4533: F, t886: F, t2770: F, t1579: F, t2828: F, t10989: F, t10992: F, t10998: F, t11000: F, t11004: F, t11013: F, t11017: F, t11019: F, t11022: F, t2765: F, t4487: F, t4534: F, t865: F) -> F {
    let t15029 = t4533 * t886;
    let t15030 = t2770 * t15029;
    let t15038 = t2770 * t1579 * t2828;
    let t15044 = F::new(0.54878743191129263322e-2) * t10989 + F::new(0.10975748638225852664e-1) * t10992 + F::new(0.19514881078765566038e-1) * t10998 - F::new(0.14634331517634470219e-1) * t11000 + F::new(0.13009920719177044025e-2) * t11004 + F::new(0.26341796731742046394e1) * t865 * t15030 - F::new(0.13170898365871023197e1) * t2765 * t4534 + F::new(0.26341796731742046394e1) * t2765 * t4487 + F::new(0.13170898365871023197e1) * t865 * t15038 - F::new(0.2601984143835408805e-1) * t11013 + t11017 + F::new(0.23131639038696784278e-2) * t11019 + F::new(0.9757440539382783019e-2) * t11022;
    t15044
}
