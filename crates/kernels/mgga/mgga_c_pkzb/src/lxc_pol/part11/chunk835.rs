//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 835/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk835<F: Float>(t237: F, t9409: F, t9449: F, t9506: F, t9527: F, t3591: F, t5490: F, t5493: F, t721: F, t730: F, t9463: F, t9336: F, t9338: F, t9345: F, t9347: F, t9350: F, t9354: F, t9358: F, t9361: F, t9363: F, t9365: F, t9367: F, t9392: F, t9394: F, t9396: F, t9400: F) -> (F, F, F, F, F, F, F) {
    let t9530 = t237 * (t9409 + t9449 + t9506 + t9527);
    let t9531 = t5490 * t3591;
    let t9532 = t5493 * t721;
    let t9533 = t9531 * t9532;
    let t9535 = 0.10254018858216406658e4 * t730 * t9533;
    let t9537 = 0.19751673498613801407e-1 * t237 * t9463;
    let t9538 = t9336 + t9338 - t9345 - t9347 + t9350 - t9354 + t9358 - t9361 + t9363 - t9365 + t9367 + t9392 + t9394 - t9396 + t9400 + t9530 - t9535 + t9537;
    (t9530, t9531, t9532, t9533, t9535, t9537, t9538)
}
