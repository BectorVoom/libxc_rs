//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1316/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1316<F: Float>(t31051: F, t4028: F, t26114: F, t8323: F, t26179: F, t31069: F, t7458: F, t25994: F, t8526: F, t1874: F, t90400: F, t8327: F) -> (F, F, F, F, F, F, F) {
    let t120051 = t4028 * t31051;
    let t120053 = t26114 * t8323;
    let t120055 = t26179 * t8323;
    let t120057 = t7458 * t31069;
    let t120063 = F::cast_from(4.0_f64) * t8526 * t25994;
    let t120064 = t90400 * t1874;
    let t120067 = F::cast_from(2.0_f64) * t26114 * t8327;
    (t120051, t120053, t120055, t120057, t120063, t120064, t120067)
}
