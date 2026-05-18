//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 775/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk775<F: Float>(t1825: F, t452: F, t4572: F, t3103: F, t979: F, t488: F, t103: F, t4495: F, t379: F, t1902: F, t4607: F, t8372: F) -> (F, F, F, F) {
    let t16044 = t452 * t1825 * t4572;
    let t16047 = t3103 * t979;
    let t16049 = t452 * t488 * t16047;
    let t16052 = t103 * t4495;
    let t16053 = t16052 * t379;
    let t16054 = t1902 * t16053;
    let t16057 = t8372 * t4607;
    (t16044, t16049, t16054, t16057)
}
