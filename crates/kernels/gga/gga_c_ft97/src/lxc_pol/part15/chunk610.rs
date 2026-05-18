//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 610/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk610<F: Float>(t11043: F, t11076: F, t100: F, t8275: F, t103: F, t7763: F, t7800: F, t1851: F, t358: F, t1073: F, t8680: F, t1570: F, t2266: F) -> (F, F, F, F, F, F, F, F) {
    let t11939 = F::new(4.0) / F::new(27.0) * t11043;
    let t11949 = F::new(4.0) / F::new(9.0) * t11076;
    let t11987 = t8275 * t100;
    let t11988 = t103 * t7763;
    let t12020 = t103 * t7800;
    let t12045 = t1851 * t358;
    let t12112 = t8680 * t1073;
    let t12116 = t2266 * t1570;
    (t11939, t11949, t11987, t11988, t12020, t12045, t12112, t12116)
}
