//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 841/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk841<F: Float>(t5248: F, t5253: F, t5258: F, t5263: F, t5274: F, t5278: F, t5282: F, t5288: F, t5295: F, t5298: F, t5302: F, t5303: F) -> F {
    let t8901 = t5248 - F::new(0.4051561992e0) * t5253 + F::new(0.10254018858216406658e4) * t5258 + t5263 + t5274 - t5278 + t5282 - t5288 - t5295 + t5298 + t5302 + F::new(0.17315859105681463759e2) * t5303;
    t8901
}
