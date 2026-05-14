//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 627/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk627<F: Float>(t645: F, t67: F, t7185: F, t4971: F, t638: F, t1751: F, t1758: F, t2436: F, t2442: F, t340: F, t6141: F, t642: F, t6707: F) -> (F, F, F) {
    let t646 = t645 < -0.66725e-1;
    let t7186 = t67 * t7185;
    let t7196 = t638 * t4971;
    let t7201 = piecewise3(t646, 0.0, 10.0 / 9.0 * t340 * t7186 * t642 - 10.0 / 27.0 * t340 * t2436 * t1758 - 10.0 / 27.0 * t340 * t1751 * t2442 + 40.0 / 81.0 * t6141 * t7196 * t6707);
    (t7186, t7196, t7201)
}
