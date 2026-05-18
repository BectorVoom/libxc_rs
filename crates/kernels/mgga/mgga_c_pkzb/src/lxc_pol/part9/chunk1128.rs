//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1128/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1128<F: Float>(t1419: F, t19418: F, t19427: F, t19435: F, t19439: F, t19444: F, t19446: F, t19450: F, t19455: F, t19458: F, t23: F, t2490: F, t2494: F, t434: F, t4816: F, t6655: F, t6662: F, t6668: F, t7: F, t980: F) -> F {
    let t19461 = F::new(10.0) * t23 * t19418 + F::new(880.0) / F::new(27.0) * t1419 * t2490 + F::new(440.0) / F::new(9.0) * t1419 * t2494 - F::new(80.0) / F::new(9.0) * t434 * t6662 + F::new(10.0) / F::new(9.0) * t7 * t19427 + F::new(80.0) / F::new(27.0) * t434 * t6655 + F::new(40.0) * t434 * t6668 + F::new(40.0) / F::new(81.0) * t7 * t19435 + F::new(10.0) / F::new(3.0) * t7 * t19439 + t19444 + F::new(40.0) / F::new(81.0) * t23 * t19446 - F::new(10.0) / F::new(3.0) * t23 * t19450 - t19455 - F::new(80.0) / F::new(9.0) * t980 * t4816 + F::new(10.0) / F::new(9.0) * t23 * t19458;
    t19461
}
