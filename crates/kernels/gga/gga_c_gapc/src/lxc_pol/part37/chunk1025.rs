//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1025/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1025<F: Float>(t11834: F, t3137: F, t7191: F, t818: F, t959: F, t11769: F, t9703: F, t3751: F, t9422: F, t11579: F, t11849: F, t2493: F, t11853: F, t19204: F, t2578: F, t3757: F, t9638: F) -> (F, F, F, F, F, F) {
    let t33808 = t11834 * t3137 * t818 * t959 * t7191;
    let t33810 = t11769 * t9703;
    let t33812 = t3751 * t9422;
    let t33815 = t11849 * t11579 * t2493;
    let t33818 = t2578 * t19204 * t11853;
    let t33820 = t3757 * t9638;
    (t33808, t33810, t33812, t33815, t33818, t33820)
}
