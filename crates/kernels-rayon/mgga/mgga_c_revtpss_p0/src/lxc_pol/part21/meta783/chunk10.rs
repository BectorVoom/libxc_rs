//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2819/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2819(t2444: f64, t4534: f64, t689: f64, t10977: f64, t10978: f64, t1579: f64, t2770: f64, t41115: f64, t41118: f64, t41125: f64, t41129: f64, t4474: f64, t51727: f64, t51729: f64, t51731: f64, t51733: f64, t51739: f64, t51742: f64, t51746: f64, t51750: f64, t51756: f64, t865: f64) -> f64 {
    let t51759 = t689 * t2444 * t4534;
    let t51762 = -t51727 - 0.65854491829355115984e-1_f64 * t51729 - 0.29272321618148349057e-1_f64 * t51731 + 0.26019841438354088051e-2_f64 * t51733 + 0.13170898365871023197e1_f64 * t865 * t2770 * t1579 * t10977 - 0.29272321618148349057e-1_f64 * t51739 + t51742 + 0.39029762157531132075e-1_f64 * t41115 - 0.29272321618148349057e-1_f64 * t51746 - 0.32927245914677557992e-1_f64 * t51750 + 0.33133632253434461091e-3_f64 * t41118 - 0.65854491829355115987e0_f64 * t4474 * t10978 + 0.19514881078765566037e-2_f64 * t41125 - 0.39029762157531132075e-2_f64 * t51756 + 0.32927245914677557992e-1_f64 * t51759 - 0.9757440539382783019e-2_f64 * t41129;
    t51762
}
