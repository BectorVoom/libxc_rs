//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1064/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1064<F: Float>(t10405: F, t10408: F, t10411: F, t10448: F, t10451: F, t10478: F, t12227: F, t1413: F, t1449: F, t16036: F, t16047: F, t2481: F, t2528: F, t28792: F, t28817: F, t3311: F, t3340: F, t3356: F, t430: F, t448: F, t459: F, t4772: F, t4828: F, t8615: F, t8705: F, t987: F, t995: F) -> (F,) {
    let t28856 = -0.99375e-1 * t1413 * t987 * t8705 - 0.33125e-1 * t1413 * t10448 * t459 - 0.99375e-1 * t12227 * t8615 + 0.165625e-1 * t430 * (t28792 + t28817) + 0.298125e0 * t4772 * t10408 * t459 + 0.298125e0 * t4772 * t10411 * t459 - 0.11925e1 * t16036 * t10405 * t459 + 0.59625e0 * t4772 * t3311 * t2528 + 0.165625e-1 * t2481 * t10448 - 0.33125e-1 * t1413 * t10478 * t448 + 0.496875e-1 * t1449 * t10478 * t459 - 0.3975e0 * t16036 * t10451 * t448 + 0.99375e0 * t16047 * t10451 * t459 - 0.59625e0 * t4828 * t3340 * t2528 + 0.1490625e0 * t1449 * t2528 * t3356 + 0.1490625e0 * t1449 * t995 * t8705;
    (t28856,)
}
