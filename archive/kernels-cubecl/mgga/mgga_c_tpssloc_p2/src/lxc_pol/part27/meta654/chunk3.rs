//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2285/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2285<F: Float>(t1385: F, t22633: F, t22635: F, t90516: F, t7692: F, t81186: F, t26338: F, t81228: F, t81326: F, t6888: F, t7691: F, t80707: F) -> (F, F, F, F) {
    let t90519 = t22633 * t22635 * t90516 * t1385;
    let t90521 = t81186 * t7692;
    let t90524 = t81228 * t81326 * t26338;
    let t90525 = F::cast_from(0.16449340668482264365e-1_f64) * t90524;
    let t90527 = t6888 * t80707 * t7691;
    (t90519, t90521, t90525, t90527)
}
