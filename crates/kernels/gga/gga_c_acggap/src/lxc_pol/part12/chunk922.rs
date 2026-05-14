//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 922/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk922<F: Float>(t1350: F, t1992: F, t30147: F, t7586: F, t142: F, t4495: F, t7436: F, t4479: F, t8888: F, t5129: F, t7647: F, t5133: F, t2001: F, t4518: F, t4667: F, t5267: F) -> (F, F, F, F, F, F, F, F) {
    let t34526 = t30147 * t7586 * t1992 * t1350;
    let t34529 = t7436 * t142 * t4495;
    let t34532 = t8888 * t142 * t4479;
    let t34534 = t7647 * t5129;
    let t34537 = t7647 * t5133;
    let t34539 = t2001 * t4518;
    let t34541 = t2001 * t4667;
    let t34543 = t2001 * t5267;
    (t34526, t34529, t34532, t34534, t34537, t34539, t34541, t34543)
}
