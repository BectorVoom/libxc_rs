//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1080/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1080<F: Float>(t16592: F, t16599: F, t16607: F, t16612: F, t19743: F, t19748: F, t19751: F, t19752: F, t28958: F, t28959: F, t28960: F, t28961: F, t28962: F, t28963: F, t28966: F, t28967: F, t28968: F) -> (F,) {
    let t29115 = -t28958 + t28959 + t28960 + t28961 - t16592 - t28962 - t28963 + t16599 - t28966 + t19743 + t16607 - t16612 - t19748 + t28967 - t28968 + t19751 + t19752;
    (t29115,)
}
