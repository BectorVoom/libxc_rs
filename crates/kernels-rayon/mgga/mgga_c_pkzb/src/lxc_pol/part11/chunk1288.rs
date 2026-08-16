//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1288/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1288(t11213: f64, t2320: f64, t11290: f64, t6317: f64, t2197: f64, t3070: f64, t3765: f64, t1185: f64, t9837: f64, t11293: f64, t6137: f64, t1184: f64, t2240: f64, t26880: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t31357 = t11213 * t2320;
    let t31369 = 6.0_f64 * t6317 * t11290;
    let t31372 = 6.0_f64 * t2197 * t3070 * t3765;
    let t31375 = 6.0_f64 * t2197 * t1185 * t9837;
    let t31377 = 0.48245938496077605201e2_f64 * t6137 * t11293;
    let t31380 = 0.48245938496077605201e2_f64 * t2240 * t26880 * t1184;
    (t31357, t31369, t31372, t31375, t31377, t31380)
}
