//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 663/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk663<F: Float>(t5586: F, t822: F, t1964: F, t4416: F, t1985: F, t2012: F, t1: F, t5514: F, t787: F, t1980: F, t2032: F) -> (F, F, F, F, F) {
    let t5662 = t822 * t5586;
    let t5665 = t1964 * t4416;
    let t5666 = t822 * t5665;
    let t5669 = t2012 * t1985;
    let t5672 = t5514 * t1;
    let t5673 = t787 * t5672;
    let t5676 = t1980 * t2032;
    (t5662, t5666, t5669, t5673, t5676)
}
