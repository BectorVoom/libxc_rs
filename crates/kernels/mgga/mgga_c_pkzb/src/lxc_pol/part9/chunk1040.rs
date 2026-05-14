//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1040/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1040<F: Float>(t1448: F, t448: F, t1444: F, t452: F, t1413: F, t1449: F, t1450: F, t1466: F, t16047: F, t19417: F, t19461: F, t2484: F, t2507: F, t2510: F, t2513: F, t2528: F, t430: F, t459: F, t4769: F, t4772: F, t4828: F, t4829: F, t4856: F, t6634: F, t6639: F, t6642: F, t6645: F, t6692: F, t987: F, t995: F) -> (F,) {
    let t19467 = t448 * t1448;
    let t19470 = t1444 * t452;
    let t19495 = -0.99375e-1 * t1413 * t6692 * t459 - 0.99375e-1 * t1413 * t2507 * t1466 - 0.33125e-1 * t1413 * t987 * t4856 + 0.165625e-1 * t430 * (t19417 + t19461) - 0.99375e-1 * t4769 * t2528 + 0.298125e0 * t19467 * t6639 - 0.99375e-1 * t19470 * t2484 - 0.19875e0 * t6634 * t6642 - 0.99375e-1 * t6634 * t6645 + 0.298125e0 * t4772 * t2510 * t1466 + 0.298125e0 * t4772 * t2507 * t1450 + 0.1490625e0 * t1449 * t2528 * t1466 + 0.496875e-1 * t1449 * t995 * t4856 + 0.99375e0 * t16047 * t995 * t4829 - 0.59625e0 * t4828 * t2513 * t1466;
    (t19495,)
}
