//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 242/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk242<F: Float>(t1410: F, t65: F, t1409: F, t43: F, t46: F, t48: F, rho1: F) -> (F, F, F, F) {
    let t1411 = t1410 * t65;
    let t1414 = t43 * t1409;
    let t1417 = t46 * rho1;
    let t1419 = F::cast_from(1.0_f64) / t48 / t1417;
    (t1411, t1414, t1417, t1419)
}
