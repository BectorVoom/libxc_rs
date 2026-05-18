//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 679/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk679<F: Float>(t158: F, t3903: F, t1255: F, t2428: F, t2029: F, t3874: F) -> (F, F, F, F) {
    let t3904 = t3903 * t158;
    let t3909 = t1255 * t1255;
    let t3910 = t2428 * t3909;
    let t3913 = t3874 * t2029;
    (t3904, t3909, t3910, t3913)
}
