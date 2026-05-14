//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 959/1294 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk959<F: Float>(t20368: F, t20369: F, t4786: F, t6575: F, t2293: F, t447: F, t1564: F, t579: F, t4390: F, t4398: F, t10524: F, t1415: F, t2371: F, t4360: F, t4803: F, t6715: F) -> (F, F, F, F, F, F, F, F) {
    let t20370 = t20368 * t20369;
    let t20374 = t4786 * t6575;
    let t20395 = t2293 * t447;
    let t20441 = t579 * t1564;
    let t20445 = t4398 * t4390;
    let t20471 = t1415 * t10524;
    let t20481 = t4360 * t2371;
    let t20496 = t4803 * t6715;
    (t20370, t20374, t20395, t20441, t20445, t20471, t20481, t20496)
}
