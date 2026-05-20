//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2731/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2731<F: Float>(t20900: F, t73: F, t12987: F, t5390: F, t12772: F, t17736: F, t21309: F, t3767: F, t70629: F, t474: F, t6593: F, t3089: F) -> (F, F, F, F, F, F) {
    let t70944 = t20900 * t73;
    let t70959 = t12987 * t5390;
    let t70982 = t17736 * t12772 * t21309;
    let t70990 = t3767 * t70629;
    let t70993 = t474 * t6593;
    let t70994 = t70993 * t3089;
    (t70944, t70959, t70982, t70990, t70993, t70994)
}
