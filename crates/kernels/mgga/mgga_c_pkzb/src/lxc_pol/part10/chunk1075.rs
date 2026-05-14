//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1075/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1075<F: Float>(t2860: F, t2875: F, t3591: F, t5498: F, t2874: F, t730: F, t5493: F, t721: F, t2801: F, t2820: F, t5845: F, t7324: F, t7486: F, t9213: F, t9215: F, t9218: F, t9221: F, t9224: F, t9227: F, t9231: F, t9234: F, t9238: F) -> (F, F, F, F, F, F, F) {
    let t9396 = 0.34631718211362927517e2 * t2860 * t2875;
    let t9397 = t5498 * t3591;
    let t9398 = t9397 * t2874;
    let t9400 = 0.10389515463408878255e3 * t730 * t9398;
    let t9401 = t3591 * t5493;
    let t9402 = t9401 * t721;
    let t9409 = 0.10254018858216406658e4 * t5845 * t9402 + t9213 - t9215 - t9218 + t9221 + t9224 + t9227 - t9231 - t9234 - t9238 - 4.0 * t7486 * t2801 + 0.64327917994770140268e2 * t7324 * t2820;
    (t9396, t9397, t9398, t9400, t9401, t9402, t9409)
}
