//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 359/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk359<F: Float>(t1424: F, t747: F, t743: F, t193: F, t6109: F, t1434: F, t1435: F, t681: F, t1433: F, t92: F) -> (F, F, F, F, F) {
    let t6110 = t1424 * t747;
    let t6111 = t743 * t6110;
    let t6113 = t6109 * t193 * t6111;
    let t6116 = t1434 * t681 * t1435;
    let t6117 = t6116 / F::new(18.0);
    let t6118 = t1433 * t92;
    (t6111, t6113, t6116, t6117, t6118)
}
