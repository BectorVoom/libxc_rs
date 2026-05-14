//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1042/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1042<F: Float>(t19442: F, t34: F, t19453: F, t38: F, t1453: F, t19418: F, t19427: F, t19446: F, t19450: F, t19458: F, t2490: F, t2494: F, t454: F, t4812: F, t4816: F, t4820: F, t4835: F, t6662: F, t6665: F, t974: F, t991: F) -> (F,) {
    let t19545 = 20.0 * t34 * t19442;
    let t19551 = 20.0 * t38 * t19453;
    let t19570 = 50.0 / 81.0 * t991 * t4812 - 25.0 / 9.0 * t991 * t4820 + t19545 + 40.0 / 81.0 * t38 * t19446 - 10.0 / 3.0 * t38 * t19450 - t19551 + 10.0 / 9.0 * t38 * t19458 + 400.0 / 27.0 * t1453 * t2490 + 200.0 / 9.0 * t1453 * t2494 - 50.0 / 9.0 * t454 * t6662 + 10.0 / 9.0 * t34 * t19427 - 50.0 / 9.0 * t991 * t4816 + 10.0 * t38 * t19418 - 2200.0 / 81.0 * t4835 * t974 - 25.0 / 3.0 * t454 * t6665;
    (t19570,)
}
