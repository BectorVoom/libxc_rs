//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 726/1129 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk726<F: Float>(t1504: F, t2982: F, t9228: F, t1875: F, t9128: F, t2983: F, t1027: F, t1781: F, t144: F, t2974: F, t7676: F, t3094: F, t1587: F, t2980: F, t1: F, t8785: F) -> (F, F, F, F, F, F, F, F) {
    let t9229 = t2982 * t1504;
    let t9230 = t9228 * t9229;
    let t9232 = t1875 * t9128;
    let t9233 = t9232 * t2983;
    let t9235 = t1027 * t1781;
    let t9238 = t7676 * t144 * t2974;
    let t9239 = t3094 * t9238;
    let t9241 = t2982 * t1587;
    let t9242 = t2980 * t9241;
    let t9244 = t8785 * t1;
    (t9229, t9230, t9233, t9235, t9239, t9241, t9242, t9244)
}
