//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 1030/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk1030<F: Float>(t2153: F, t2206: F, t5692: F, t8: F, t5: F, t17890: F, t277: F, t575: F, t8596: F, t2468: F, t3263: F, t2902: F, t423: F) -> (F, F, F, F, F, F, F) {
    let t24625 = t2153 * t2206;
    let t24759 = F::new(1.0) / t8 / t5692;
    let t24760 = t5 * t24759;
    let t24761 = t277 * t17890;
    let t24906 = t8596 * t575;
    let t24915 = t3263 * t2468;
    let t24980 = t2902 * t423;
    (t24625, t24759, t24760, t24761, t24906, t24915, t24980)
}
