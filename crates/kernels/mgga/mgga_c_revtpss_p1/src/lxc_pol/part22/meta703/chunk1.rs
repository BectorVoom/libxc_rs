//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2719/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2719<F: Float>(t1357: F, t6919: F, t689: F, t1444: F, t6918: F, t4076: F, t14081: F, t14084: F, t14087: F, t1424: F, t14299: F, t1904: F, t22395: F, t22400: F, t22405: F, t22407: F, t9677: F, t9687: F, t9691: F) -> (F, F, F, F) {
    let t22409 = t1357 * t6919;
    let t22410 = t689 * t22409;
    let t22414 = t6918 * t1444;
    let t22415 = t4076 * t22414;
    let t22418 = F::cast_from(0.26341796731742046394e1_f64) * t1424 * t22395 - F::cast_from(0.9757440539382783019e-2_f64) * t22400 - F::cast_from(0.11565819519348392139e-2_f64) * t9677 + F::cast_from(0.13009920719177044025e-1_f64) * t9687 + F::cast_from(0.9757440539382783019e-2_f64) * t22405 - t14081 + t14084 - F::cast_from(0.19514881078765566037e-1_f64) * t22407 + F::cast_from(0.54878743191129263322e-2_f64) * t22410 + t14087 - t9691 - F::cast_from(0.13170898365871023197e1_f64) * t14299 * t1904 + F::cast_from(0.13170898365871023197e1_f64) * t1424 * t22415;
    (t22409, t22414, t22415, t22418)
}
