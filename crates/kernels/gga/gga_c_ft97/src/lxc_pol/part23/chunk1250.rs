//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1250/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1250<F: Float>(t109414: F, t109481: F, t17794: F, t1091: F, t24437: F, t24438: F, t27850: F, t18459: F, t24531: F, t24432: F, t6118: F, t2574: F, t5120: F, t6119: F, t713: F, t108188: F, t3875: F, t96934: F, t96935: F) -> (F, F, F, F, F, F) {
    let t124020 = t109414 * t109481 * t17794;
    let t124026 = t24437 * t24438 * t27850 * t1091;
    let t124029 = t24531 * t18459;
    let t124031 = t6118 * t24432 * t124029;
    let t124036 = t24437 * t2574 * t6119 * t5120 * t713;
    let t124040 = t96934 * t96935 * t108188 * t3875;
    (t124020, t124026, t124029, t124031, t124036, t124040)
}
