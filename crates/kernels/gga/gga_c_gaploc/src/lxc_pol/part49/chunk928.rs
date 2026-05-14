//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 928/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk928<F: Float>(t13729: F, t6305: F, t38392: F, t874: F, t2268: F, t2343: F, t1358: F, t13777: F, t2299: F, t488: F, t1365: F, t38272: F, t6525: F, t426: F, t46849: F, t535: F) -> (F, F, F, F, F, F) {
    let t47024 = t6305 * t13729;
    let t47026 = t38392 * t874;
    let t47028 = t2268 * t2343 * t47026;
    let t47032 = t1358 * t2299 * t13777 * t488;
    let t47036 = t6525 * t1365 * t38272;
    let t47040 = t2268 * t535 * t46849 * t426;
    (t47024, t47026, t47028, t47032, t47036, t47040)
}
