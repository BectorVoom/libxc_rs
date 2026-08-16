//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1132/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1132(t42: f64, t4823: f64, t1413: f64, t1444: f64, t1449: f64, t1450: f64, t1466: f64, t16036: f64, t16074: f64, t19539: f64, t19570: f64, t2481: f64, t2484: f64, t2507: f64, t2510: f64, t2513: f64, t2528: f64, t448: f64, t453: f64, t459: f64, t4772: f64, t4828: f64, t4829: f64, t6631: f64, t6634: f64, t6692: f64, t6700: f64, t6703: f64, t6747: f64, t987: f64, t995: f64) -> f64 {
    let t19579 = t4823 * t42;
    let t19603 = -0.11925e1_f64 * t16036 * t2510 * t1450 + 0.59625e0_f64 * t4772 * t6700 * t459 + 0.298125e0_f64 * t4772 * t6703 * t459 + 0.298125e0_f64 * t4772 * t2484 * t1466 - 0.165625e-1_f64 * t453 * (t19539 + t19570) - 0.3975e0_f64 * t16036 * t987 * t4829 + 0.496875e-1_f64 * t2481 * t6692 + 0.165625e-1_f64 * t19579 * t987 + 0.496875e-1_f64 * t6631 * t2507 - 0.99375e-1_f64 * t1413 * t6747 * t448 - 0.99375e-1_f64 * t1413 * t2528 * t1444 - 0.99375e-1_f64 * t6634 * t6703 + 0.298125e0_f64 * t16074 * t2513 - 0.33125e-1_f64 * t1413 * t995 * t4823 - 0.59625e0_f64 * t4828 * t2528 * t1450 + 0.1490625e0_f64 * t1449 * t6747 * t459;
    t19603
}
