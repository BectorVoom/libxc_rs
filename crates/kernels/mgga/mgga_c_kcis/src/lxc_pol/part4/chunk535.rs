//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 535/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk535<F: Float>(t2855: F, t922: F, t1021: F, t1020: F, t2820: F, t304: F, t86: F) -> (F, F, F, F) {
    let t2856 = t2855 * t922;
    let t2857 = t1021 * t2856;
    let t2858 = t1020 * t2857;
    let t2861 = t86 * t2820 * t304;
    (t2856, t2857, t2858, t2861)
}
