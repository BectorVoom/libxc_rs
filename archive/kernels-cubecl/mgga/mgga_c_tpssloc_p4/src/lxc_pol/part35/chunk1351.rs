//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1351/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1351<F: Float>(t19451: F, t7468: F, t1983: F, t2019: F, t74064: F, t28813: F, t7685: F, t28821: F, t7754: F, t1845: F, t6463: F, t26161: F, t26162: F) -> (F, F, F, F, F) {
    let t105181 = F::cast_from(6.0_f64) * t19451 * t7468;
    let t105184 = F::cast_from(6.0_f64) * t1983 * t2019 * t74064;
    let t105186 = F::cast_from(6.0_f64) * t7685 * t28813;
    let t105188 = F::cast_from(3.0_f64) * t28821 * t7754;
    let t105189 = t6463 * t1845;
    let t105192 = F::cast_from(6.0_f64) * t26161 * t26162 * t105189;
    (t105181, t105184, t105186, t105188, t105192)
}
