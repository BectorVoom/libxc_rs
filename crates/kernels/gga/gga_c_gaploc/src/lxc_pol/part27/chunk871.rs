//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 871/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk871<F: Float>(t10276: F, t1358: F, t8042: F, t921: F, t2358: F, t8045: F, t2497: F, t2798: F, t1016: F, t6553: F, t2801: F, t6556: F, t2355: F, t2902: F, t3366: F, t4342: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t10278 = 0.94850022118920498663e-2 * t1358 * t10276;
    let t10285 = t8042 * t921;
    let t10286 = t8045 * t2358;
    let t10287 = 2.0 * t10286;
    let t10288 = t2798 * t2497;
    let t10289 = t6553 * t1016;
    let t10290 = t6556 * t2801;
    let t10291 = 2.0 * t10290;
    let t10292 = t2355 * t2902;
    let t10293 = t4342 * t3366;
    (t10278, t10285, t10286, t10287, t10288, t10289, t10290, t10291, t10292, t10293)
}
