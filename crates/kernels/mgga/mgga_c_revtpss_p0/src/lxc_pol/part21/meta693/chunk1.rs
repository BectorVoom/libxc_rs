//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2515/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2515<F: Float>(t1203: F, t12626: F, t225: F, t12967: F, t12995: F, t12627: F, t1269: F, t3566: F, t3727: F, t12640: F, t44842: F, t487: F) -> (F, F, F, F, F, F, F) {
    let t45384 = t1203 * t12626;
    let t45385 = t45384 * t225;
    let t45389 = t12967 * t12995;
    let t45427 = t12627 * t1269;
    let t45430 = t3566 * t3727;
    let t45433 = t12640 * t1269;
    let t45438 = t44842 * t487;
    (t45384, t45385, t45389, t45427, t45430, t45433, t45438)
}
