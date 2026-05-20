//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2819/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2819<F: Float>(t2444: F, t4534: F, t689: F, t10977: F, t10978: F, t1579: F, t2770: F, t41115: F, t41118: F, t41125: F, t41129: F, t4474: F, t51727: F, t51729: F, t51731: F, t51733: F, t51739: F, t51742: F, t51746: F, t51750: F, t51756: F, t865: F) -> F {
    let t51759 = t689 * t2444 * t4534;
    let t51762 = -t51727 - F::cast_from(0.65854491829355115984e-1_f64) * t51729 - F::cast_from(0.29272321618148349057e-1_f64) * t51731 + F::cast_from(0.26019841438354088051e-2_f64) * t51733 + F::cast_from(0.13170898365871023197e1_f64) * t865 * t2770 * t1579 * t10977 - F::cast_from(0.29272321618148349057e-1_f64) * t51739 + t51742 + F::cast_from(0.39029762157531132075e-1_f64) * t41115 - F::cast_from(0.29272321618148349057e-1_f64) * t51746 - F::cast_from(0.32927245914677557992e-1_f64) * t51750 + F::cast_from(0.33133632253434461091e-3_f64) * t41118 - F::cast_from(0.65854491829355115987e0_f64) * t4474 * t10978 + F::cast_from(0.19514881078765566037e-2_f64) * t41125 - F::cast_from(0.39029762157531132075e-2_f64) * t51756 + F::cast_from(0.32927245914677557992e-1_f64) * t51759 - F::cast_from(0.9757440539382783019e-2_f64) * t41129;
    t51762
}
