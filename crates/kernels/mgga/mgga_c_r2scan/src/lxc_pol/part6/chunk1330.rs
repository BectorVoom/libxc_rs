//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1330/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1330<F: Float>(t410: F, t7124: F, t406: F, t7008: F, t41: F, t457: F, t7007: F, t4911: F, t899: F, t4982: F, t7030: F, t2788: F, t4973: F, t19694: F, t19698: F, t19702: F, t19709: F, t19712: F, t20180: F, t23982: F, t23984: F, t23986: F, t23992: F) -> (F, F, F, F, F, F, F, F) {
    let t25031 = t410 * t7124;
    let t25032 = 24.0 * t25031;
    let t25033 = t406 * t7008;
    let t25034 = 12.0 * t25033;
    let t25036 = t41 * t7007 * t457;
    let t25037 = 3.0 * t25036;
    let t25038 = t4911 * t899;
    let t25039 = 24.0 * t25038;
    let t25040 = t4982 * t899;
    let t25041 = 144.0 * t25040;
    let t25042 = t410 * t7030;
    let t25043 = 12.0 * t25042;
    let t25044 = t2788 * t4973;
    let t25045 = 0.21687162600603479684e-1 * t25044;
    let t25046 = t23982 - t23984 - t19694 + t19698 + t23986 + t19702 - t23992 - t25032 + t25034 + t25037 + t19709 + t25039 - t25041 - t19712 - t25043 - t20180 - t25045;
    (t25032, t25034, t25037, t25039, t25041, t25043, t25045, t25046)
}
