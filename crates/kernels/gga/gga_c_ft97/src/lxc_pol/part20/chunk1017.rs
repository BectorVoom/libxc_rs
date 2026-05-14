//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1017/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1017<F: Float>(t1882: F, t24575: F, t2492: F, t6148: F, t38953: F, t6076: F, t24698: F, t6168: F, t8232: F, t6177: F, t6154: F, t1445: F, t89: F, t9555: F, t24798: F, t8392: F) -> (F, F, F, F, F, F, F, F, F) {
    let t97725 = t1882 * t24575;
    let t97733 = t2492 * t6148;
    let t97740 = t38953 * t6076;
    let t97745 = t1882 * t24698;
    let t97770 = t8232 * t6168;
    let t97772 = t8232 * t6177;
    let t97777 = t2492 * t6154;
    let t97790 = 28.0 / 81.0 * t89 * t9555 * t1445;
    let t97791 = t8392 * t24798;
    (t97725, t97733, t97740, t97745, t97770, t97772, t97777, t97790, t97791)
}
