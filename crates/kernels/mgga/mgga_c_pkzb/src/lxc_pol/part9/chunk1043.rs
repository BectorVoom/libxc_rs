//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1043/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1043<F: Float>(t42: F, t4823: F, t1413: F, t1444: F, t1449: F, t1450: F, t1466: F, t16036: F, t16074: F, t19539: F, t19570: F, t2481: F, t2484: F, t2507: F, t2510: F, t2513: F, t2528: F, t448: F, t453: F, t459: F, t4772: F, t4828: F, t4829: F, t6631: F, t6634: F, t6692: F, t6700: F, t6703: F, t6747: F, t987: F, t995: F) -> (F,) {
    let t19579 = t4823 * t42;
    let t19603 = -0.11925e1 * t16036 * t2510 * t1450 + 0.59625e0 * t4772 * t6700 * t459 + 0.298125e0 * t4772 * t6703 * t459 + 0.298125e0 * t4772 * t2484 * t1466 - 0.165625e-1 * t453 * (t19539 + t19570) - 0.3975e0 * t16036 * t987 * t4829 + 0.496875e-1 * t2481 * t6692 + 0.165625e-1 * t19579 * t987 + 0.496875e-1 * t6631 * t2507 - 0.99375e-1 * t1413 * t6747 * t448 - 0.99375e-1 * t1413 * t2528 * t1444 - 0.99375e-1 * t6634 * t6703 + 0.298125e0 * t16074 * t2513 - 0.33125e-1 * t1413 * t995 * t4823 - 0.59625e0 * t4828 * t2528 * t1450 + 0.1490625e0 * t1449 * t6747 * t459;
    (t19603,)
}
