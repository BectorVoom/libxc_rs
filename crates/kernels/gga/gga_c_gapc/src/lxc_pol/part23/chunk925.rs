//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 925/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk925<F: Float>(t1040: F, t11388: F, t1026: F, t424: F, t1046: F, t8: F, t8652: F, t667: F) -> (F, F, F, F, F) {
    let t11389 = t11388 * t1040;
    let t11391 = t424 * t1026;
    let t11392 = t11391 * t1046;
    let t11395 = F::new(1.0) / t8 / t8652;
    let t11397 = t667 * t11395 * M_PI;
    (t11389, t11391, t11392, t11395, t11397)
}
