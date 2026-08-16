//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1176/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1176<F: Float>(t11769: F, t9703: F, t3751: F, t9422: F, t11579: F, t11849: F, t2493: F, t11853: F, t19204: F, t2578: F, t3757: F, t9638: F) -> (F, F, F, F, F) {
    let t33810 = t11769 * t9703;
    let t33812 = t3751 * t9422;
    let t33815 = t11849 * t11579 * t2493;
    let t33818 = t2578 * t19204 * t11853;
    let t33820 = t3757 * t9638;
    (t33810, t33812, t33815, t33818, t33820)
}
