//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2174/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2174<F: Float>(t22574: F, t56194: F, t8643: F, t12461: F, t6995: F, t26161: F, t26163: F, t22581: F, t7685: F, t24987: F, t7000: F, t25985: F, t6876: F) -> (F, F, F, F, F) {
    let t90029 = F::cast_from(6.0_f64) * t22574 * t8643 * t56194;
    let t90031 = t6995 * t12461;
    let t90034 = F::cast_from(4.0_f64) * t26161 * t90031 * t26163;
    let t90036 = F::cast_from(2.0_f64) * t7685 * t22581;
    let t90038 = F::cast_from(2.0_f64) * t24987 * t7000;
    let t90040 = F::cast_from(6.0_f64) * t6876 * t25985;
    (t90029, t90034, t90036, t90038, t90040)
}
