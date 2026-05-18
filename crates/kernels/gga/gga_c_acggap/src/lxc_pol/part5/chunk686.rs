//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 686/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk686<F: Float>(t4806: F, t721: F, t1060: F, t1072: F, t495: F, t3126: F, t3124: F, t3143: F, t503: F, t1049: F, t1476: F, t1298: F, t301: F) -> (F, F, F, F, F, F, F, F, F) {
    let t4807 = t4806 * t721;
    let t4808 = t1060 * t4807;
    let t4809 = F::new(0.12225e0) * t4808;
    let t4810 = t1072 * t495;
    let t4811 = t4810 * t3126;
    let t4812 = t3124 * t4811;
    let t4814 = t3143 * t503;
    let t4816 = t1049 * t1476;
    let t4817 = F::new(0.1956e1) * t4816;
    let t4818 = t1298 * t301;
    (t4807, t4808, t4809, t4811, t4812, t4814, t4816, t4817, t4818)
}
