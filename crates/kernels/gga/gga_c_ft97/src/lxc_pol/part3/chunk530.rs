//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 530/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk530<F: Float>(t3886: F, t4140: F, t4139: F, t1212: F, t312: F, t684: F, t2874: F, t1248: F, t870: F, t2881: F, t1250: F, t1882: F) -> (F, F, F, F, F, F, F) {
    let t4141 = t4140 * t3886;
    let t4142 = t4139 * t4141;
    let t4145 = t312 * t1212;
    let t4146 = t4145 * t684;
    let t4147 = t2874 * t4146;
    let t4150 = t870 * t1248;
    let t4151 = t4150 * t684;
    let t4152 = t2881 * t4151;
    let t4156 = t1882 * t1250;
    (t4141, t4142, t4146, t4147, t4151, t4152, t4156)
}
