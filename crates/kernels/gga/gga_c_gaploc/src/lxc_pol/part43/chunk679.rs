//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 679/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk679<F: Float>(t712: F, t3221: F, t12390: F, t5337: F, t5340: F, t5345: F, t5348: F, t1692: F, t3222: F, t12380: F, t713: F, t928: F) -> (F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t12557 = pi * t712;
    let t12558 = t3221 * t12557;
    let t12561 = t12390 * t5337 * t5340;
    let t12564 = t5345 * t12390 * t5348;
    let t12566 = t1692 * t3222;
    let t12568 = t713 * t12380;
    let t12569 = t12568 * t928;
    (t12557, t12558, t12561, t12564, t12566, t12568, t12569)
}
