//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 780/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk780<F: Float>(t3103: F, t942: F, t110: F, t1871: F, t376: F, t4547: F, t89: F, t4495: F, t452: F, t499: F, t15885: F, t986: F) -> (F, F, F, F, F) {
    let t16120 = t942 * t3103;
    let t16122 = t1871 * t110 * t16120;
    let t16126 = t89 * t376 * t4547;
    let t16129 = t452 * t499 * t4495;
    let t16133 = t452 * t110 * t15885;
    let t16137 = t452 * t986 * t3103;
    (t16122, t16126, t16129, t16133, t16137)
}
