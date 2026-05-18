//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 591/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk591<F: Float>(t1435: F, t983: F, t444: F, t27: F, t8: F, t1429: F, t23: F, t2490: F, t2494: F, t434: F, t445: F, t7: F, t974: F, t980: F) -> (F, F, F, F, F) {
    let t2499 = t1435 * t983;
    let t2500 = t2499 * t444;
    let t2503 = t27 * t8;
    let t2504 = t2503 * t1429;
    let t2507 = -F::new(40.0) / F::new(9.0) * t434 * t974 + F::new(10.0) / F::new(9.0) * t7 * t2490 + F::new(5.0) / F::new(3.0) * t7 * t2494 - F::new(40.0) / F::new(9.0) * t980 * t445 + F::new(10.0) / F::new(9.0) * t23 * t2500 - F::new(5.0) / F::new(3.0) * t23 * t2504;
    (t2499, t2500, t2503, t2504, t2507)
}
