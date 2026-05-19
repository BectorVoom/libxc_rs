//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 841/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk841<F: Float>(t5248: F, t5253: F, t5258: F, t5263: F, t5274: F, t5278: F, t5282: F, t5288: F, t5295: F, t5298: F, t5302: F, t5303: F) -> F {
    let t8901 = t5248 - F::cast_from(0.4051561992e0_f64) * t5253 + F::cast_from(0.10254018858216406658e4_f64) * t5258 + t5263 + t5274 - t5278 + t5282 - t5288 - t5295 + t5298 + t5302 + F::cast_from(0.17315859105681463759e2_f64) * t5303;
    t8901
}
