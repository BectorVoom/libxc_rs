//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 768/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk768<F: Float>(t9030: F, t9031: F, t1027: F, t1787: F, t1740: F, t9016: F, t9020: F, t19: F, t424: F, t3114: F, t3117: F, t3123: F, t8888: F) -> (F, F, F, F, F, F, F) {
    let t9032 = t9030 * t9031;
    let t9034 = t1027 * t1787;
    let t9036 = t9016 * t1740;
    let t9038 = t9020 * t1740;
    let t9040 = t424 * t19;
    let t9041 = t9040 * t3114;
    let t9042 = t9041 * t3117;
    let t9044 = t8888 * t3123;
    (t9032, t9034, t9036, t9038, t9041, t9042, t9044)
}
