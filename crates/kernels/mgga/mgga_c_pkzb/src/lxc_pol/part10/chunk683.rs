//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 683/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk683<F: Float>(t133: F, t2916: F, t793: F, t2036: F, t2968: F, t2923: F, t2970: F) -> (F, F, F, F) {
    let t2976 = t2916 * t133;
    let t2977 = t2976 * t793;
    let t2980 = t2036 * t2968;
    let t2981 = t2970 * t2923;
    (t2976, t2977, t2980, t2981)
}
