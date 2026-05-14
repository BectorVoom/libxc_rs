//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1019/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1019<F: Float>(t31081: F, t31096: F, t258: F, t242: F, t31061: F, t4973: F, t6161: F, t2606: F, t4965: F, t3891: F, t4969: F, t24668: F, t5073: F, t14127: F, t1424: F, t5181: F, t729: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t31097 = t31081 + t31096;
    let t31098 = t31097 * t258;
    let t31102 = t242 * t31061;
    let t31106 = t6161 * t4973;
    let t31107 = t2606 * t31106;
    let t31110 = t6161 * t4965;
    let t31111 = t3891 * t31110;
    let t31114 = t6161 * t4969;
    let t31115 = t2606 * t31114;
    let t31118 = t24668 * t5073;
    let t31119 = t14127 * t31118;
    let t31123 = t729 * t5181 * t1424;
    (t31097, t31098, t31102, t31106, t31107, t31110, t31111, t31114, t31115, t31118, t31119, t31123)
}
