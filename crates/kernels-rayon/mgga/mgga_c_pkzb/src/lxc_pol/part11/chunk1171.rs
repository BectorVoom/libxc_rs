//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1171/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1171(t10405: f64, t10408: f64, t10411: f64, t10448: f64, t10451: f64, t10478: f64, t12227: f64, t1413: f64, t1449: f64, t16036: f64, t16047: f64, t2481: f64, t2528: f64, t28792: f64, t28817: f64, t3311: f64, t3340: f64, t3356: f64, t430: f64, t448: f64, t459: f64, t4772: f64, t4828: f64, t8615: f64, t8705: f64, t987: f64, t995: f64) -> f64 {
    let t28856 = -0.99375e-1_f64 * t1413 * t987 * t8705 - 0.33125e-1_f64 * t1413 * t10448 * t459 - 0.99375e-1_f64 * t12227 * t8615 + 0.165625e-1_f64 * t430 * (t28792 + t28817) + 0.298125e0_f64 * t4772 * t10408 * t459 + 0.298125e0_f64 * t4772 * t10411 * t459 - 0.11925e1_f64 * t16036 * t10405 * t459 + 0.59625e0_f64 * t4772 * t3311 * t2528 + 0.165625e-1_f64 * t2481 * t10448 - 0.33125e-1_f64 * t1413 * t10478 * t448 + 0.496875e-1_f64 * t1449 * t10478 * t459 - 0.3975e0_f64 * t16036 * t10451 * t448 + 0.99375e0_f64 * t16047 * t10451 * t459 - 0.59625e0_f64 * t4828 * t3340 * t2528 + 0.1490625e0_f64 * t1449 * t2528 * t3356 + 0.1490625e0_f64 * t1449 * t995 * t8705;
    t28856
}
