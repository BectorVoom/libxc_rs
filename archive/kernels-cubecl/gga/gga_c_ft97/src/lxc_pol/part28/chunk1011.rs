//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1011/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1011<F: Float>(t22940: F, t6557: F, t22914: F, t34553: F, t7211: F, t984: F, t34613: F, t92: F, t1286: F, t34580: F, t376: F, t25542: F, t7162: F) -> (F, F, F, F, F, F) {
    let t144704 = t22940 * t6557;
    let t144708 = t22914 * t34553;
    let t144714 = t7211 * t984;
    let t144719 = t34613 * t92;
    let t144725 = t1286 * t376 * t34580;
    let t144727 = t7162 * t25542;
    (t144704, t144708, t144714, t144719, t144725, t144727)
}
