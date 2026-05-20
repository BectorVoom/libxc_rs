//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2043/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2043<F: Float>(t1828: F, t5497: F, t3737: F, t1269: F, t6628: F, t3783: F, t3769: F, t1280: F, t20703: F, t1287: F, t1811: F, t5284: F) -> (F, F, F, F, F, F) {
    let t21407 = t1828 * t5497;
    let t21408 = t3737 * t21407;
    let t21415 = t1269 * t6628;
    let t21416 = t21415 * t3783;
    let t21427 = t21415 * t3769;
    let t21430 = t1280 * t20703;
    let t21436 = t1811 * t5284 * t1287;
    (t21407, t21408, t21416, t21427, t21430, t21436)
}
