//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1011/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1011<F: Float>(t252: F, t776: F, t829: F, t6646: F, t22986: F, t2627: F, t6604: F) -> (F, F, F, F, F) {
    let t22987 = t252 * t776;
    let t22988 = t22987 * t829;
    let t22989 = t6646 * t22988;
    let t22990 = t22986 * t22989;
    let t22996 = t6604 * t2627;
    (t22987, t22988, t22989, t22990, t22996)
}
