//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 977/1034 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk977<F: Float>(t26977: F, t6535: F, t22561: F, t7042: F, t114422: F, t26161: F, t26558: F, t31304: F, t6880: F, t1874: F, t84097: F, t31537: F, t7057: F) -> (F, F, F, F, F, F) {
    let t115231 = F::cast_from(4.0_f64) * t26977 * t6535;
    let t115233 = F::cast_from(4.0_f64) * t7042 * t22561;
    let t115238 = F::cast_from(4.0_f64) * t26161 * t26558 * t114422;
    let t115245 = F::cast_from(6.0_f64) * t31304 * t6880;
    let t115249 = F::cast_from(2.0_f64) * t84097 * t1874;
    let t115251 = F::cast_from(4.0_f64) * t31537 * t7057;
    (t115231, t115233, t115238, t115245, t115249, t115251)
}
