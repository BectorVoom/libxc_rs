//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 849/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk849<F: Float>(t32011: F, t925: F, t1564: F, t32019: F, t7824: F, t22943: F, t6547: F, t7274: F, t979: F, t8418: F, t1332: F, t6557: F) -> (F, F, F, F, F, F) {
    let t34552 = t32011 * t925;
    let t34553 = t1564 * t34552;
    let t34557 = t7824 * t32019 * t925;
    let t34560 = t22943 * t6547;
    let t34562 = t7274 * t979;
    let t34563 = t8418 * t34562;
    let t34565 = t1332 * t6557;
    (t34553, t34557, t34560, t34562, t34563, t34565)
}
