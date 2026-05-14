//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1045/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1045<F: Float>(t24: F, t1429: F, t1652: F, t1655: F, t507: F, t82: F, t2551: F, t500: F, t1003: F, t16250: F, t1651: F, t2548: F, t5106: F, t5107: F, t5113: F, t6782: F, t6785: F, t8: F, t91: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t90 = t24 <= zeta_threshold;
    let t19660 = t1429 * t1652;
    let t19663 = t507 * t1655;
    let t19669 = t82 * t507;
    let t19672 = t1429 * t1655;
    let t19680 = 32.0 * t2551 * t500;
    let t19682 = piecewise3(t90, 0.0, 40.0 / 81.0 * t16250 * t1003 * t5107 + 16.0 / 9.0 * t5106 * t8 * t19660 - 8.0 / 9.0 * t6782 * t19663 - 8.0 / 3.0 * t1651 * t1429 * t507 + 8.0 * t6785 * t19669 - 8.0 / 3.0 * t6785 * t19672 + 4.0 / 9.0 * t2548 * t5113 + 16.0 * t91 * t82 - t19680);
    (t19660, t19663, t19669, t19672, t19682)
}
