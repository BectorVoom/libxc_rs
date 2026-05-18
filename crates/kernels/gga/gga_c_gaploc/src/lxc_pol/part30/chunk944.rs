//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 944/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk944<F: Float>(t10286: F, t2497: F, t2798: F, t1016: F, t6553: F, t2801: F, t6556: F, t2355: F, t2902: F, t3366: F, t4342: F, t605: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10287 = F::new(2.0) * t10286;
    let t10288 = t2798 * t2497;
    let t10289 = t6553 * t1016;
    let t10290 = t6556 * t2801;
    let t10291 = F::new(2.0) * t10290;
    let t10292 = t2355 * t2902;
    let t10293 = t4342 * t3366;
    let t10294 = F::new(2.0) * t10293;
    let t10295 = t3366 * t605;
    (t10287, t10288, t10289, t10290, t10291, t10292, t10293, t10294, t10295)
}
