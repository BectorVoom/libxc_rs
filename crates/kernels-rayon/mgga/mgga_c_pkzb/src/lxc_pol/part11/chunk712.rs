//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 712/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk712(t5152: f64, t555: f64, t1508: f64, t1511: f64, t1675: f64, t191: f64, t1545: f64, t546: f64, t513: f64, t1542: f64, t1548: f64, t1705: f64, t575: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t5154 = 0.10254018858216406658e4_f64 * t555 * t5152;
    let t5158 = t1511 * t1508;
    let t5165 = 1.0_f64 / t1675 / t191;
    let t5177 = t1545 * t546;
    let t5178 = 36.0_f64 * t5177;
    let t5179 = t1545 * t513;
    let t5186 = 60.0_f64 * t1542 * t546;
    let t5187 = t1548 * t513;
    let t5189 = t1542 * t513;
    let t5221 = t575 * t1705;
    (t5154, t5158, t5165, t5177, t5178, t5179, t5186, t5187, t5189, t5221)
}
