//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 619/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk619<F: Float>(t199: F, t4982: F, t1783: F, t636: F, t1841: F, t735: F, t1648: F, t1898: F, t155: F, t589: F) -> (F, F, F, F, F) {
    let t4984 = 2.0 / 15.0 * t4982 * t199;
    let t4985 = t1783 * t636;
    let t4986 = 8.0 / 15.0 * t4985;
    let t4987 = t1841 * t735;
    let t4990 = 8.0 / 15.0 * t1648 * t1898;
    let t4991 = t155 * t589;
    (t4984, t4986, t4987, t4990, t4991)
}
