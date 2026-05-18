//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 565/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk565<F: Float>(t4010: F, t4025: F, t59: F, t85: F, t1357: F, t807: F, t2635: F, t2644: F, t2835: F, t1390: F, t224: F, t2841: F) -> (F, F, F, F, F, F, F, F) {
    let t4027 = (t4010 + t4025) * t59;
    let t4028 = t4027 * t85;
    let t4029 = F::new(0.19751673498613801407e-1) * t4028;
    let t4030 = t1357 * t807;
    let t4031 = F::new(0.24415263074675393405e-3) * t4030;
    let t4032 = F::new(24.0) * t2635;
    let t4036 = F::new(2.0) * t2644;
    let t4038 = F::new(0.23392894490538584828e1) * t2835;
    let t4039 = t224 * t1390;
    let t4040 = F::new(8.0) * t4039;
    let t4041 = F::new(16.0) * t2841;
    (t4027, t4029, t4031, t4032, t4036, t4038, t4040, t4041)
}
