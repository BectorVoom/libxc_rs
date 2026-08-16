//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 845/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk845<F: Float>(t107: F, t2760: F, t1415: F, t1359: F, t2875: F, t544: F, t4820: F, t7906: F, t1339: F, t2754: F, t590: F, t2792: F) -> (F, F, F, F, F, F, F) {
    let t8261 = t2760 * t107;
    let t8262 = t1415 * t8261;
    let t8265 = t1359 * t2875;
    let t8266 = t544 * t8265;
    let t8269 = t4820 * t7906;
    let t8272 = t1339 * t2754;
    let t8273 = t8272 * t590;
    let t8278 = t2792 * t590;
    (t8262, t8265, t8266, t8269, t8272, t8273, t8278)
}
