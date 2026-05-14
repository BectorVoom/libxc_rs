//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 964/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk964<F: Float>(t2029: F, t759: F, t2106: F, t7700: F, t178: F, t5711: F, t2020: F) -> (F, F, F, F, F) {
    let t7701 = t2029 * t759;
    let t7702 = t7701 * t2106;
    let t7703 = t7700 * t7702;
    let t7706 = t5711 * t178;
    let t7707 = t2020 * t7706;
    (t7701, t7702, t7703, t7706, t7707)
}
