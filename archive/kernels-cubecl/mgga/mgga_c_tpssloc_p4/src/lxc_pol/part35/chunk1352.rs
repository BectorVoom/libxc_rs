//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1352/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1352<F: Float>(t1799: F, t6324: F, t22574: F, t26162: F, t1873: F, t22425: F, t652: F, t28827: F, t7685: F, t23035: F, t25224: F, t28298: F) -> (F, F, F, F) {
    let t105201 = t1799 * t6324;
    let t105204 = F::cast_from(18.0_f64) * t22574 * t26162 * t105201;
    let t105207 = F::cast_from(2.0_f64) * t652 * t22425 * t1873;
    let t105213 = F::cast_from(18.0_f64) * t7685 * t28827;
    let t105223 = t23035 * t25224 * t28298;
    (t105204, t105207, t105213, t105223)
}
