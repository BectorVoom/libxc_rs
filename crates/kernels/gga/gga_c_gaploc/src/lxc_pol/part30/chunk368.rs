//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 368/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk368<F: Float>(t1628: F, t601: F, t570: F, t575: F, t1589: F, t494: F, t561: F, t566: F) -> (F, F, F, F, F) {
    let t1629 = t1628 * t601;
    let t1632 = t1628 * t570;
    let t1635 = t1628 * t575;
    let t1638 = t1589 * t494;
    let t1641 = t561 * t566;
    (t1629, t1632, t1635, t1638, t1641)
}
