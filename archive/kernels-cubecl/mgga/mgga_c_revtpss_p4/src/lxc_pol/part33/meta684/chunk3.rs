//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2254/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2254<F: Float>(t1256: F, t30812: F, t104988: F, t104990: F, t20298: F, t20302: F, t21008: F, t21022: F, t21121: F, t21161: F, t21219: F, t21228: F, t26867: F, t29047: F, t29054: F, t6640: F, t97149: F, t97232: F) -> F {
    let t112491 = t30812 * t1256;
    let t112515 = -F::cast_from(0.30488190661738479624e-2_f64) * t112491 + F::cast_from(0.10162730220579493208e-2_f64) * t104988 - F::cast_from(0.17149607247227894789e-2_f64) * t97149 * t21121 + F::cast_from(0.47637797908966374413e-3_f64) * t26867 * t21008 - F::cast_from(0.57165357490759649296e-3_f64) * t26867 * t21161 - F::cast_from(0.57165357490759649296e-3_f64) * t97232 * t6640 - F::cast_from(0.57165357490759649296e-3_f64) * t26867 * t21228 - F::cast_from(0.57165357490759649296e-3_f64) * t26867 * t21022 - F::cast_from(0.28582678745379824648e-3_f64) * t26867 * t21219 + t29047 * t29054 * t20302 / F::cast_from(108.0_f64) + t29047 * t29054 * t20298 / F::cast_from(36.0_f64) + t104990 / F::cast_from(648.0_f64);
    t112515
}
