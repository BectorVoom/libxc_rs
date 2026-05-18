//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 794/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk794<F: Float>(t4598: F, t973: F, t1628: F, t2704: F, t2710: F, t1589: F, t2586: F, t2027: F, t2628: F, t6138: F, t959: F, t5673: F) -> (F, F, F, F, F, F, F) {
    let t7545 = t4598 * t973;
    let t7550 = t1628 * t2704;
    let t7553 = t1628 * t2710;
    let t7558 = t1589 * t2586;
    let t7563 = t2027 * t2628;
    let t7565 = t6138 * t959;
    let t7567 = t5673 * t959;
    (t7545, t7550, t7553, t7558, t7563, t7565, t7567)
}
