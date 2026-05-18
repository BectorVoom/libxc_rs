//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 592/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk592<F: Float>(t448: F, t995: F, t459: F, t2490: F, t2494: F, t2500: F, t2504: F, t34: F, t38: F, t445: F, t454: F, t974: F, t991: F) -> (F, F, F) {
    let t2510 = t995 * t448;
    let t2513 = t995 * t459;
    let t2528 = -F::new(25.0) / F::new(9.0) * t454 * t974 + F::new(10.0) / F::new(9.0) * t34 * t2490 + F::new(5.0) / F::new(3.0) * t34 * t2494 - F::new(25.0) / F::new(9.0) * t991 * t445 + F::new(10.0) / F::new(9.0) * t38 * t2500 - F::new(5.0) / F::new(3.0) * t38 * t2504;
    (t2510, t2513, t2528)
}
