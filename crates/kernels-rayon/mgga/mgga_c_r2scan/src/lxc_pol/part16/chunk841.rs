//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 841/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk841(t5248: f64, t5253: f64, t5258: f64, t5263: f64, t5274: f64, t5278: f64, t5282: f64, t5288: f64, t5295: f64, t5298: f64, t5302: f64, t5303: f64) -> f64 {
    let t8901 = t5248 - 0.4051561992e0_f64 * t5253 + 0.10254018858216406658e4_f64 * t5258 + t5263 + t5274 - t5278 + t5282 - t5288 - t5295 + t5298 + t5302 + 0.17315859105681463759e2_f64 * t5303;
    t8901
}
