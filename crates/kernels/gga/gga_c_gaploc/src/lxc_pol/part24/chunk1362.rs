//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1362/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1362<F: Float>(t1441: F, t34267: F, t590: F, t30247: F, t30251: F, t30253: F, t30261: F, t30263: F, t30265: F, t30288: F, t30294: F, t34256: F, t34258: F, t34260: F, t34261: F, t34262: F, t34263: F, t34266: F) -> F {
    let t34270 = F::cast_from(0.2044956050875773316e1_f64) * t1441 * t34267 * t590;
    let t34271 = t34256 + t34258 - t30247 - t30251 + t30253 - t30261 - t34260 + t30263 - t30265 - t30288 + t30294 + t34261 + t34262 + t34263 + t34266 + t34270;
    t34271
}
