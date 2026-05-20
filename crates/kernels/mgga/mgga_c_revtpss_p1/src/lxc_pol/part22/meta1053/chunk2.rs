//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3721/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3721<F: Float>(t17237: F, t17381: F, t5381: F, t57270: F, t57273: F, t57290: F, t57292: F, t57295: F, t57297: F, t57299: F, t57314: F, t57316: F, t57318: F, t57321: F, t57382: F) -> F {
    let t70565 = -F::new(2.0) / F::new(243.0) * t57270 + t57273 / F::new(324.0) + F::cast_from(0.85748036236139473944e-3_f64) * t57382 * t17381 + t57290 / F::new(162.0) + t57292 / F::new(81.0) - t57295 / F::new(432.0) - F::cast_from(0.15244095330869239812e-2_f64) * t57297 + F::cast_from(0.28582678745379824648e-3_f64) * t57299 - F::cast_from(0.1270341277572436651e-2_f64) * t5381 * t17237 - F::cast_from(0.30488190661738479624e-2_f64) * t57314 + F::cast_from(0.57165357490759649296e-3_f64) * t57316 - F::cast_from(0.30488190661738479624e-2_f64) * t57318 + F::cast_from(0.19055119163586549765e-2_f64) * t57321;
    t70565
}
