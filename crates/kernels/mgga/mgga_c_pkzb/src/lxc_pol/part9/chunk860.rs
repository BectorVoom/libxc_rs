//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 860/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk860<F: Float>(t12: F, t24: F, t1642: F, t8: F, t1429: F, t439: F, t1643: F, t1646: F, t2540: F, t2543: F, t6767: F, t82: F, t87: F, t1003: F, t5106: F, t1651: F, t507: F, t1652: F, t1655: F, t2548: F, t2551: F, t91: F, zeta_threshold: F) -> (F, F, F, F, F, F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t6770 = t1642 * t8;
    let t6771 = t1429 * t439;
    let t6781 = piecewise3(t84, 0.0, -8.0 / 27.0 * t6767 * t1643 + 16.0 / 9.0 * t6770 * t6771 + 4.0 / 9.0 * t2540 * t1646 + 8.0 / 3.0 * t87 * t1429 - 8.0 * t2543 * t82);
    let t6782 = t5106 * t1003;
    let t6785 = t1651 * t8;
    let t6786 = t1429 * t507;
    let t6796 = piecewise3(t90, 0.0, -8.0 / 27.0 * t6782 * t1652 - 16.0 / 9.0 * t6785 * t6786 + 4.0 / 9.0 * t2548 * t1655 - 8.0 / 3.0 * t91 * t1429 + 8.0 * t2551 * t82);
    (t6770, t6771, t6781, t6782, t6785, t6786, t6796)
}
