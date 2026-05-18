//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1195/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1195<F: Float>(t7494: F, t7497: F, t7488: F, t1025: F, t2630: F, t7242: F, t21874: F, t221: F, t454: F, t2640: F, t7245: F, t2634: F) -> (F, F, F, F, F, F) {
    let t22132 = t7497 * t7494;
    let t22134 = t7497 * t7488;
    let t22138 = F::new(0.38025319932552508021e2) * t2630 * t1025 * t7242;
    let t22141 = F::new(0.11483599538271604938e-1) * t221 * t21874 * t454;
    let t22148 = t7245 * t2640;
    let t22150 = t2634 * t2634;
    (t22132, t22134, t22138, t22141, t22148, t22150)
}
