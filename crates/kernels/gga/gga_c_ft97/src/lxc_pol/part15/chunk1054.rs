//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1054/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1054<F: Float>(t4702: F, t4710: F, t1013: F, t126: F, t4441: F, t4466: F, t39538: F, t85413: F, t2007: F, t85424: F, t528: F, t85568: F) -> (F, F, F, F, F, F, F) {
    let t86694 = t4702 * t4702;
    let t86701 = t4710 * t4710;
    let t86708 = t1013 * t4710;
    let t86741 = t4466 * t4441 * t126;
    let t86744 = t39538 * t85413;
    let t86747 = t2007 * t85424;
    let t86750 = t528 * t85568;
    (t86694, t86701, t86708, t86741, t86744, t86747, t86750)
}
