//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1224/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1224(t1083: f64, t1899: f64, t25873: f64, t2782: f64, t9228: f64, t25908: f64, t2751: f64, t25832: f64, t2787: f64, t7483: f64, t9225: f64, t7411: f64, t9229: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30216 = 0.48245938496077605201e2_f64 * t1899 * t25873 * t1083;
    let t30219 = 0.48245938496077605201e2_f64 * t1899 * t9228 * t2782;
    let t30221 = 6.0_f64 * t25908 * t2751;
    let t30223 = 0.48245938496077605201e2_f64 * t25832 * t2787;
    let t30225 = 6.0_f64 * t7483 * t9225;
    let t30227 = 0.48245938496077605201e2_f64 * t7411 * t9229;
    (t30216, t30219, t30221, t30223, t30225, t30227)
}
