//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1394/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1394<F: Float>(t1005: F, t3579: F, t1434: F, t25432: F, t25436: F, t260: F, t29598: F, t29627: F, t29629: F, t29631: F, t29633: F, t29635: F, t29637: F, t29639: F, t29640: F, t29644: F, t29648: F, t29694: F, t29741: F, t29988: F, t30045: F, t30098: F, t30216: F, t3583: F) -> (F, F) {
    let t30221 = t3579 * t1005;
    let t30228 = t260 * (t29598 + t29640 + t29694 + t29741 + t29988 + t30045 + t30098 + t30216) + F::cast_from(0.4155806185363551302e3_f64) * t25436 * t3583 * t30221 - F::cast_from(0.14035736694323150897e2_f64) * t25432 * t1434 * t30221 - t29627 + t29629 + t29631 - t29633 - t29635 - t29637 + t29639 - t29644 + t29648;
    (t30221, t30228)
}
