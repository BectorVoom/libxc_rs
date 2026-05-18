//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 789/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk789<F: Float>(t1264: F, t5056: F, t247: F, t3629: F, t5351: F, t3626: F, t3627: F, t471: F, t1715: F, t1227: F, t1261: F, t1266: F, t1808: F, t3625: F, t3647: F, t3686: F, t3705: F, t5373: F, t5379: F, t5381: F, t5384: F, t5386: F, t5391: F) -> (F, F, F, F, F, F, F) {
    let t5396 = t1264 * t5056;
    let t5397 = t247 * t5396;
    let t5401 = t5351 * t3629;
    let t5402 = t3626 * t5401;
    let t5405 = t3627 * t471;
    let t5406 = t1715 * t5405;
    let t5407 = t3626 * t5406;
    let t5410 = t5373 * t1227 / F::new(108.0) - t3686 / F::new(864.0) - F::new(0.95275595817932748827e-4) * t5379 - F::new(0.14291339372689912324e-3) * t5381 * t1266 + F::new(0.42874018118069736972e-3) * t5384 * t5386 + F::new(0.7622047665434619906e-3) * t5391 * t1266 - F::new(0.14291339372689912324e-3) * t3647 * t1808 - F::new(0.14291339372689912324e-3) * t1261 * t5397 + F::new(0.14291339372689912324e-3) * t3705 - F::new(0.14291339372689912324e-3) * t3625 * t5402 - F::new(0.14291339372689912324e-3) * t3625 * t5407;
    (t5397, t5401, t5402, t5405, t5406, t5407, t5410)
}
