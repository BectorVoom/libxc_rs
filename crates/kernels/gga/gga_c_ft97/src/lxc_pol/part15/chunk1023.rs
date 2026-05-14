//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1023/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1023<F: Float>(t43495: F, t88252: F, t89: F, t9716: F, t446: F, t835: F, t88606: F, t1212: F, t21351: F, t43468: F, t10758: F, t88612: F, t4965: F, t5299: F, t10409: F, t5225: F) -> (F, F, F, F, F, F, F, F) {
    let t89865 = t89 * t9716 * t43495 * t88252;
    let t89868 = t446 * t835 * t88606;
    let t89870 = t21351 * t1212;
    let t89872 = t446 * t43468 * t89870;
    let t89875 = t446 * t10758 * t88612;
    let t89877 = t4965 * t5299;
    let t89879 = t446 * t10409 * t89877;
    let t89881 = t4965 * t5225;
    (t89865, t89868, t89870, t89872, t89875, t89877, t89879, t89881)
}
