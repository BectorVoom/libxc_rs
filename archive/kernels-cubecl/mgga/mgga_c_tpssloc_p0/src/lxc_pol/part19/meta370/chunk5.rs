//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1376/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1376<F: Float>(t42145: F, t42148: F, t42233: F, t42235: F, t42238: F, t42241: F, t42692: F, t42697: F, t42699: F, t42701: F, t42704: F, t42708: F, t42712: F) -> F {
    let t43642 = -t42692 + t42697 + t42699 - t42701 - t42704 - t42145 + t42148 - t42708 - t42712 - t42233 + t42235 - t42238 - t42241;
    t43642
}
