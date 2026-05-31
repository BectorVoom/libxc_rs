//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1089/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1089<F: Float>(t3626: F, t5406: F, t1227: F, t1261: F, t1266: F, t1808: F, t3625: F, t3647: F, t3686: F, t3705: F, t5373: F, t5379: F, t5381: F, t5384: F, t5386: F, t5391: F, t5397: F, t5402: F) -> (F, F) {
    let t5407 = t3626 * t5406;
    let t5410 = t5373 * t1227 / F::cast_from(108.0_f64) - t3686 / F::cast_from(864.0_f64) - F::cast_from(0.95275595817932748827e-4_f64) * t5379 - F::cast_from(0.14291339372689912324e-3_f64) * t5381 * t1266 + F::cast_from(0.42874018118069736972e-3_f64) * t5384 * t5386 + F::cast_from(0.7622047665434619906e-3_f64) * t5391 * t1266 - F::cast_from(0.14291339372689912324e-3_f64) * t3647 * t1808 - F::cast_from(0.14291339372689912324e-3_f64) * t1261 * t5397 + F::cast_from(0.14291339372689912324e-3_f64) * t3705 - F::cast_from(0.14291339372689912324e-3_f64) * t3625 * t5402 - F::cast_from(0.14291339372689912324e-3_f64) * t3625 * t5407;
    (t5407, t5410)
}
