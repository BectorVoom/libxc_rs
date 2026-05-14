//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1275/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1275<F: Float>(t22414: F, t4076: F, t14081: F, t14084: F, t14087: F, t1424: F, t14299: F, t1904: F, t22395: F, t22400: F, t22405: F, t22407: F, t22410: F, t9677: F, t9687: F, t9691: F) -> (F,) {
    let t22415 = t4076 * t22414;
    let t22418 = 0.26341796731742046394e1 * t1424 * t22395 - 0.9757440539382783019e-2 * t22400 - 0.11565819519348392139e-2 * t9677 + 0.13009920719177044025e-1 * t9687 + 0.9757440539382783019e-2 * t22405 - t14081 + t14084 - 0.19514881078765566037e-1 * t22407 + 0.54878743191129263322e-2 * t22410 + t14087 - t9691 - 0.13170898365871023197e1 * t14299 * t1904 + 0.13170898365871023197e1 * t1424 * t22415;
    (t22418,)
}
