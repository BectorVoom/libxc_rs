//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1016/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1016<F: Float>(t24723: F, t8392: F, t24790: F, t1451: F, t3281: F, t1882: F, t24820: F, t24823: F, t24395: F, t258: F, t24753: F, t24817: F, t6148: F, t737: F, t24668: F, t53798: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t97561 = t8392 * t24723;
    let t97584 = t8392 * t24790;
    let t97629 = 28.0 / 81.0 * t3281 * t1451;
    let t97637 = t1882 * t24820;
    let t97639 = t1882 * t24823;
    let t97676 = t258 * t24395;
    let t97681 = t8392 * t24753;
    let t97683 = t8392 * t24817;
    let t97701 = t737 * t6148;
    let t97705 = t53798 * t24668;
    (t97561, t97584, t97629, t97637, t97639, t97676, t97681, t97683, t97701, t97705)
}
