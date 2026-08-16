//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 882/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk882(t237: f64, t9409: f64, t9449: f64, t9506: f64, t9527: f64, t3591: f64, t5490: f64, t5493: f64, t721: f64, t730: f64, t9463: f64, t9336: f64, t9338: f64, t9345: f64, t9347: f64, t9350: f64, t9354: f64, t9358: f64, t9361: f64, t9363: f64, t9365: f64, t9367: f64, t9392: f64, t9394: f64, t9396: f64, t9400: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9530 = t237 * (t9409 + t9449 + t9506 + t9527);
    let t9531 = t5490 * t3591;
    let t9532 = t5493 * t721;
    let t9533 = t9531 * t9532;
    let t9535 = 0.10254018858216406658e4_f64 * t730 * t9533;
    let t9537 = 0.19751673498613801407e-1_f64 * t237 * t9463;
    let t9538 = t9336 + t9338 - t9345 - t9347 + t9350 - t9354 + t9358 - t9361 + t9363 - t9365 + t9367 + t9392 + t9394 - t9396 + t9400 + t9530 - t9535 + t9537;
    (t9530, t9531, t9532, t9533, t9535, t9537, t9538)
}
