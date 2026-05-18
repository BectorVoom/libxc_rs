//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1046/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1046<F: Float>(t1005: F, t4557: F, t1487: F, t336: F, t1319: F, t3570: F, t1137: F, t5161: F, t3621: F, t5165: F, t3382: F, t4295: F) -> (F, F, F, F, F, F) {
    let t18121 = t1005 * t4557;
    let t18129 = t336 * t1487;
    let t18139 = t3570 * t1319;
    let t18141 = t1137 * t5161;
    let t18147 = t3621 * t5165;
    let t18153 = t3382 * t4295;
    (t18121, t18129, t18139, t18141, t18147, t18153)
}
