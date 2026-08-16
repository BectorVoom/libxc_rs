//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 756/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk756<F: Float>(t22819: F, t22842: F, t3076: F, t32187: F, t32279: F, t32281: F, t32284: F, t32289: F, t32292: F, t32295: F, t32297: F, t32301: F, t32304: F, t32308: F, t32313: F, t32316: F, t32318: F, t385: F, t399: F, t7173: F, t7181: F, t7183: F, t7202: F) -> F {
    let t32322 = F::cast_from(0.39129660776942540761e-2_f64) * t32279 * t32281 - F::cast_from(0.68116566383613497688e-3_f64) * t22819 * t32284 - F::cast_from(0.11854761295685025975e-1_f64) * t7181 * t32187 + F::cast_from(0.22227677429409423704e-2_f64) * t32289 * t7183 + F::cast_from(0.88910709717637694816e-2_f64) * t22842 * t32292 - F::cast_from(0.25845121844514357744e-4_f64) * t32295 * t32297 - F::cast_from(0.23254900946437792e-1_f64) * t32301 * t385 + F::cast_from(0.25845121844514357744e-4_f64) * t32304 * t32297 - F::cast_from(0.31303728621554032609e-1_f64) * t7202 * t32308 + t32313 + F::cast_from(0.11854761295685025975e-1_f64) * t7173 * t399 + F::cast_from(0.1443087735596363459e-7_f64) * t3076 * t32316 * t32318;
    t32322
}
