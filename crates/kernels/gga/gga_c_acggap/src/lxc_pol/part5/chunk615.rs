//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 615/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk615<F: Float>(t1629: F, t4199: F, t945: F, t1651: F, t930: F, t322: F, t407: F) -> (F, F, F, F) {
    let t4200 = t1629 * t4199;
    let t4203 = t1629 * t945;
    let t4206 = t1651 * t930;
    let t4210 = t407 * t322;
    (t4200, t4203, t4206, t4210)
}
