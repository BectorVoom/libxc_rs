//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 1048/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk1048<F: Float>(t2371: F, t4360: F, t4803: F, t6715: F, t20117: F, t6508: F, t20013: F, t1433: F, t9271: F, t1323: F, t874: F, t2366: F) -> (F, F, F, F, F, F, F) {
    let t20481 = t4360 * t2371;
    let t20496 = t4803 * t6715;
    let t20513 = t6508 * t20117;
    let t20521 = t6508 * t20013;
    let t20535 = t1433 * t9271;
    let t20539 = t874 * t1323;
    let t20540 = t2366 * t20539;
    (t20481, t20496, t20513, t20521, t20535, t20539, t20540)
}
