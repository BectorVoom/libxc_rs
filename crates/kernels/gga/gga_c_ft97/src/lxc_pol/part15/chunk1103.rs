//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1103/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1103<F: Float>(t4635: F, t4917: F, t4934: F, t18391: F, t5147: F, t1131: F, t21351: F, t41816: F, t446: F, t21369: F, t2354: F, t79697: F, t992: F) -> (F, F, F, F, F, F, F, F) {
    let t88105 = t4917 * t4635;
    let t88114 = t4917 * t4934;
    let t88131 = t18391 * t5147;
    let t88141 = t21351 * t1131;
    let t88143 = t446 * t41816 * t88141;
    let t88145 = t21369 * t1131;
    let t88147 = t446 * t2354 * t88145;
    let t88149 = t79697 * t992;
    (t88105, t88114, t88131, t88141, t88143, t88145, t88147, t88149)
}
