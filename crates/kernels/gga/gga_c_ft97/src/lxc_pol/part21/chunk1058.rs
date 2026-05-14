//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1058/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1058<F: Float>(t1736: F, t373: F, t420: F, t22632: F, t25670: F, t5598: F, t22572: F, t25709: F, t25708: F, t11119: F, t93014: F, t22619: F, t25779: F, t415: F, t26007: F, t376: F, t5665: F) -> (F, F, F, F, F, F, F) {
    let t101466 = t420 * t1736 * t373;
    let t101498 = 0.25537443351851851852e-1 * t5598 * t22632 * t25670;
    let t101504 = t22572 * t25709;
    let t101505 = t25708 * t101504;
    let t101512 = t11119 * t93014;
    let t101532 = 0.29693535778629056444e-3 * t22619 * t415 * t25779;
    let t101587 = t5665 * t376 * t26007;
    (t101466, t101498, t101504, t101505, t101512, t101532, t101587)
}
