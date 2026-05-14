//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 872/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk872<F: Float>(t789: F, t2021: F, t271: F, t2019: F, t785: F, t2009: F, t2030: F) -> (F, F, F, F, F) {
    let t5999 = t789 * t789;
    let t6000 = 1.0 / t5999;
    let t6012 = 1.0 / t2021 / t271;
    let t6017 = t2019 * t785;
    let t6022 = t2030 * t2009;
    (t5999, t6000, t6012, t6017, t6022)
}
