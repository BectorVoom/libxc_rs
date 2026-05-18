//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1172/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1172<F: Float>(t1882: F, t36175: F, t36002: F, t870: F, t875: F, t36246: F, t36211: F, t36157: F, t8392: F, t36257: F, t25253: F, t7124: F) -> (F, F, F, F, F, F, F) {
    let t154787 = t1882 * t36175;
    let t154793 = t36002 * t870;
    let t154794 = t154793 * t875;
    let t154807 = t1882 * t36246;
    let t154813 = t1882 * t36211;
    let t154820 = t8392 * t36157;
    let t154827 = t1882 * t36257;
    let t154833 = t25253 * t7124;
    (t154787, t154794, t154807, t154813, t154820, t154827, t154833)
}
