//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 789/1383 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk789<F: Float>(t1196: F, t1270: F, t198: F, t2292: F, t2302: F, t3205: F, t3209: F, t3213: F, t3216: F, t3281: F, t3304: F, t3307: F, t3310: F, t3391: F, t509: F, t5366: F, t5371: F, t5394: F, t5451: F, t5458: F) -> F {
    let t5462 = t1270 * t198 * t509 * t5451 - t198 * t3205 * t509 * t5458 + F::cast_from(3.0_f64) * t1196 * t198 * t5366 + F::cast_from(6.0_f64) * t198 * t3391 * t5371 - t2292 + t2302 - t3209 + t3213 + t3216 + t3281 - t3304 + t3307 + t3310 + t5394;
    t5462
}
