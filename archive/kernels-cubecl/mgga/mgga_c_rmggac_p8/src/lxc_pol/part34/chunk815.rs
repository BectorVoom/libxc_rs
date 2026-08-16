//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 815/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk815<F: Float>(t13862: F, t14041: F, t8615: F, t14078: F, t8659: F, t14125: F, t236: F, t68884: F, t8602: F, t495: F, t598: F, t68876: F) -> (F, F, F, F) {
    let t74655 = t14041 * t13862 * t8615;
    let t74657 = t8659 * t14078;
    let t74662 = t68884 * t14125 * t236 * t8602;
    let t74667 = t68876 * t14125 * t236 * t598 * t495;
    (t74655, t74657, t74662, t74667)
}
