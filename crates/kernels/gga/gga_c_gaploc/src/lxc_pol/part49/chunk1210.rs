//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1210/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1210<F: Float>(t13800: F, t4614: F, t574: F, t1445: F, t38413: F, t874: F, t11986: F, t2293: F, t13749: F, t1564: F, t1562: F, t475: F) -> (F, F, F, F) {
    let t48217 = t574 * t4614 * t13800;
    let t48221 = t574 * t1445 * t38413 * t874;
    let t48225 = t574 * t1445 * t11986 * t2293;
    let t48227 = t1564 * t13749;
    let t48231 = F::new(0.69017266717057349418e1) * t1562 * t1445 * t48227 * t475;
    (t48217, t48221, t48225, t48231)
}
