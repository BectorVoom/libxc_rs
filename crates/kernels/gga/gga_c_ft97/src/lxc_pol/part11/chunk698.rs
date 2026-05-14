//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 698/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk698<F: Float>(t10235: F, t829: F, t2648: F, t2744: F, t1882: F, t2667: F, t2336: F, t2671: F, t89: F, t2680: F, t683: F, t2682: F, t684: F, t446: F, t2409: F, t824: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10236 = t10235 * t829;
    let t10238 = t2648 * t2744;
    let t10243 = t1882 * t2667;
    let t10246 = t89 * t2336 * t2671;
    let t10248 = t683 * t2680;
    let t10249 = t684 * t2682;
    let t10250 = t10248 * t10249;
    let t10251 = t446 * t10250;
    let t10253 = t2409 * t824;
    (t10236, t10238, t10243, t10246, t10248, t10249, t10250, t10251, t10253)
}
