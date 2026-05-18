//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 648/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk648<F: Float>(t10915: F, t240: F, t2917: F, t342: F, t4910: F, t630: F, t1882: F, t4923: F, t4917: F, t9570: F, t9577: F, t226: F, t2383: F) -> (F, F, F, F, F, F, F) {
    let t17687 = t10915 * t240;
    let t17694 = t2917 * t240;
    let t17703 = t342 * t630 * t4910;
    let t17720 = t1882 * t4923;
    let t17748 = t9570 * t4917;
    let t17765 = t9577 * t4917;
    let t17818 = t2383 * t226;
    (t17687, t17694, t17703, t17720, t17748, t17765, t17818)
}
