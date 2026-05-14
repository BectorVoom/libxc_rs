//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 492/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk492<F: Float>(t560: F, t910: F, t551: F, t552: F, t2526: F, t133: F, t978: F, t255: F) -> (F, F, F, F, F, F) {
    let t2654 = t910 * t560;
    let t2656 = t551 * t552 * t2654;
    let t2661 = t552 * t2526;
    let t2662 = t551 * t2661;
    let t2665 = t133 * t978;
    let t2666 = t2665 * t255;
    (t2654, t2656, t2661, t2662, t2665, t2666)
}
