//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 944/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk944<F: Float>(t8862: F, t9780: F, t1052: F, t29646: F, t10105: F, t1960: F, t3689: F, t874: F) -> (F, F, F, F) {
    let t44238 = F::new(4.0) * t8862 * t9780;
    let t44239 = t29646 * t1052;
    let t44242 = F::new(2.0) * t1960 * t1052 * t10105;
    let t46849 = t3689 * t874;
    (t44238, t44239, t44242, t46849)
}
