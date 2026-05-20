//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2729/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2729<F: Float>(t1284: F, t20849: F, t3624: F, t12772: F, t17729: F, t21036: F, t3625: F, t44250: F, t6639: F, t17423: F, t21049: F, t21439: F) -> (F, F, F, F, F) {
    let t70800 = t20849 * t1284 * t3624;
    let t70806 = t17729 * t12772 * t21036;
    let t70809 = t3625 * t44250 * t6639;
    let t70811 = t21049 * t17423;
    let t70819 = t21439 * t3624;
    (t70800, t70806, t70809, t70811, t70819)
}
