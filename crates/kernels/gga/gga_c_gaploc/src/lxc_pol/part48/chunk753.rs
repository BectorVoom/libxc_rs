//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 753/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk753<F: Float>(t1022: F, t7275: F, t1: F, t32364: F, t787: F, t10954: F, t1457: F, t32356: F, t739: F, t10938: F, t2021: F, t33137: F) -> (F, F, F, F, F, F) {
    let t33360 = t7275 * t1022;
    let t33399 = t787 * t32364 * t1;
    let t33436 = t1457 * t10954;
    let t33561 = t739 * t32356;
    let t33565 = t2021 * t10938;
    let t33575 = t33137 * t1;
    (t33360, t33399, t33436, t33561, t33565, t33575)
}
