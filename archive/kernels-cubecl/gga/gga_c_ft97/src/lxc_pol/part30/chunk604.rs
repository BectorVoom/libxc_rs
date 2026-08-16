//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 604/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk604<F: Float>(t1154: F, t713: F, t2574: F, t6119: F, t24437: F, t1091: F, t2354: F, t24546: F, t6118: F, t1433: F, t3051: F, t3746: F) -> (F, F, F, F, F) {
    let t27796 = t1154 * t713;
    let t27798 = t2574 * t6119 * t27796;
    let t27799 = t24437 * t27798;
    let t27802 = t2354 * t24546 * t1091;
    let t27803 = t6118 * t27802;
    let t27805 = t1433 * t3051;
    let t27807 = t2354 * t6119 * t3746;
    (t27796, t27799, t27803, t27805, t27807)
}
