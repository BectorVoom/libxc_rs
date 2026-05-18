//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 834/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk834<F: Float>(t1564: F, t37283: F, t446: F, t432: F, t7966: F, t10: F, t11175: F, t83: F, t7788: F, t8494: F, t379: F, t8544: F) -> (F, F, F, F, F, F, F, F) {
    let t37285 = t446 * t1564 * t37283;
    let t37287 = t7966 * t432;
    let t37289 = t446 * t1564 * t37287;
    let t37292 = t10 * t11175 * t83;
    let t37293 = F::new(140.0) / F::new(243.0) * t37292;
    let t37294 = t7788 * t8494;
    let t37296 = t446 * t1564 * t37294;
    let t37298 = t8544 * t379;
    (t37285, t37287, t37289, t37292, t37293, t37294, t37296, t37298)
}
