//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 973/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk973<F: Float>(t1410: F, t2479: F, t2521: F, t3482: F, t6951: F, t3518: F, t7070: F, t3514: F, t967: F, t2478: F, t2515: F, t3517: F) -> (F, F, F, F, F, F, F, F, F) {
    let t8973 = t1410 * t2479;
    let t8975 = F::new(6.0) * t2521 * t8973;
    let t8977 = F::new(4.0) * t6951 * t3482;
    let t8979 = F::new(0.32163958997385070134e2) * t7070 * t3518;
    let t8980 = t3514 * t967;
    let t8982 = F::new(4.0) * t2478 * t8980;
    let t8983 = t1410 * t2515;
    let t8985 = F::new(2.0) * t2478 * t8983;
    let t8986 = t3517 * t2479;
    (t8973, t8975, t8977, t8979, t8980, t8982, t8983, t8985, t8986)
}
