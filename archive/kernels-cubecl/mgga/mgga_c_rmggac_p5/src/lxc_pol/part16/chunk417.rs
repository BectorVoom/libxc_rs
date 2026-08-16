//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 417/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk417<F: Float>(t4084: F, t946: F, t249: F, t973: F, t945: F, t1090: F, t1101: F, t378: F, t483: F, t7: F, t151: F, t5: F) -> (F, F, F, F, F) {
    let t4085 = t4084 * t946;
    let t4087 = t249 * t973;
    let t4089 = F::cast_from(0.16265371950452609763e-1_f64) * t945 * t4087;
    let t4101 = F::cast_from(6.0_f64) * t1090 * t378 * t1101;
    let t4103 = t7 * t483;
    let t4106 = F::cast_from(0.34450798614814814813e-2_f64) * t5 * t4103 * t151;
    (t4085, t4089, t4101, t4103, t4106)
}
