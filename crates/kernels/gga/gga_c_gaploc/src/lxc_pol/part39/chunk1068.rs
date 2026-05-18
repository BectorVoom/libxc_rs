//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 1068/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk1068<F: Float>(t8862: F, t9780: F, t1052: F, t29646: F, t10105: F, t1960: F, t3418: F, t6553: F, t10283: F, t2497: F, t13760: F, t501: F) -> (F, F, F, F, F, F) {
    let t44238 = F::new(4.0) * t8862 * t9780;
    let t44239 = t29646 * t1052;
    let t44242 = F::new(2.0) * t1960 * t1052 * t10105;
    let t44243 = t6553 * t3418;
    let t44245 = t10283 * t2497;
    let t46845 = t13760 * t501;
    (t44238, t44239, t44242, t44243, t44245, t46845)
}
