//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 828/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk828<F: Float>(t6548: F, t8428: F, t8426: F, t914: F, t1027: F, t6554: F, t1221: F, t2367: F, t3280: F, t1220: F, t1135: F, t9: F, t1122: F, t3120: F, t3116: F, t3117: F, t3126: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t8429 = t8428 * t6548;
    let t8430 = t8426 * t8429;
    let t8431 = t914 * t8430;
    let t8434 = t1027 * t6554;
    let t8435 = t1221 * t8434;
    let t8436 = t914 * t8435;
    let t8443 = t2367 * t3280;
    let t8444 = t1220 * t8443;
    let t8446 = t9 * t1135;
    let t8447 = t8446 * t1122;
    let t8448 = t8447 * t3120;
    let t8449 = t3116 * t8448;
    let t8451 = t3117 * t3126;
    (t8429, t8430, t8431, t8434, t8435, t8436, t8443, t8444, t8446, t8447, t8449, t8451)
}
