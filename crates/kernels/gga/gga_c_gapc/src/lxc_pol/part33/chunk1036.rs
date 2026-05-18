//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 1036/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk1036<F: Float>(t13738: F, t8676: F, t21: F, t3142: F, t3712: F, t8654: F, t4043: F, t1030: F, t26312: F, t20487: F, t3141: F, t3131: F, t3137: F) -> (F, F, F, F, F, F, F) {
    let t26369 = t8676 * t13738;
    let t26396 = t3712 * t3142 * t21;
    let t26416 = t8654 * M_PI;
    let t26447 = t4043 * M_PI;
    let t26561 = t1030 * t26312;
    let t26578 = t3141 * t20487;
    let t26597 = t3131 * t3137;
    (t26369, t26396, t26416, t26447, t26561, t26578, t26597)
}
