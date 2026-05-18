//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 983/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk983<F: Float>(t2524: F, t9104: F, t1410: F, t7065: F, t2473: F, t3514: F, t238: F, t3505: F, t801: F, t3509: F, t1392: F, t2466: F) -> (F, F, F, F, F, F, F, F) {
    let t9106 = F::new(0.16081979498692535067e2) * t9104 * t2524;
    let t9108 = F::new(1.0) * t7065 * t1410;
    let t9110 = F::new(2.0) * t2473 * t3514;
    let t9112 = t238 * t801 * t3505;
    let t9113 = F::new(0.32862666666666666666e0) * t9112;
    let t9115 = t238 * t801 * t3509;
    let t9116 = F::new(0.32862666666666666666e0) * t9115;
    let t9117 = t2466 * t1392;
    (t9106, t9108, t9110, t9112, t9113, t9115, t9116, t9117)
}
