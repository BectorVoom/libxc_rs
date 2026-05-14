//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 781/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk781<F: Float>(t11081: F, t3516: F, t3514: F, t414: F, t982: F, t990: F, t209: F, t287: F, t421: F, t736: F, t416: F, t1247: F, t3484: F, t1242: F, t3497: F, t1236: F, t3643: F) -> (F, F, F, F, F, F) {
    let t11082 = t11081 * t3516;
    let t11083 = t3514 * t11082;
    let t11086 = t414 * t982 * t990;
    let t11091 = t209 * t736 * t287 * t421;
    let t11093 = 5.0 / 2592.0 * t416 * t11091;
    let t11098 = t3484 * t1247;
    let t11100 = t1242 * t3497;
    let t11151 = t1236 * t3643;
    (t11083, t11086, t11093, t11098, t11100, t11151)
}
