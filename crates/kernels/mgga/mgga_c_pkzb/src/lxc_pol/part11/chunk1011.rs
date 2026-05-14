//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1011/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1011<F: Float>(t1095: F, t5830: F, t1070: F, t5775: F, t2848: F, t5493: F, t20716: F, t20748: F, t20751: F, t1915: F, t2793: F, t1088: F, t5829: F, t1107: F, t1977: F, t1954: F, t2826: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t20908 = t5830 * t1095;
    let t20911 = t1070 * t5775;
    let t20982 = t2848 * t5493;
    let t21055 = 0.20659e1 * t20716;
    let t21058 = 0.104195e1 * t20748;
    let t21059 = 0.104195e1 * t20751;
    let t21087 = t2793 * t1915;
    let t21090 = t1088 * t5829;
    let t21093 = t1977 * t1107;
    let t21143 = t2826 * t1954;
    (t20908, t20911, t20982, t21055, t21058, t21059, t21087, t21090, t21093, t21143)
}
