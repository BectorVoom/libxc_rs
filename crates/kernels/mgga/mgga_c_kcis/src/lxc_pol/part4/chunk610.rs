//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 610/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk610<F: Float>(t109: F, t287: F, t209: F, t421: F, t416: F, t25: F, t992: F, t1254: F, t1251: F, t1263: F, t286: F, t2887: F, t2844: F, t2630: F, t283: F, t414: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3495 = t109 * t287;
    let t3497 = t209 * t3495 * t421;
    let t3499 = t416 * t3497 / 864.0;
    let t3500 = t25 * t992;
    let t3501 = t3500 * t1254;
    let t3502 = t1251 * t3501;
    let t3504 = t25 * t1263;
    let t3505 = t1251 * t3504;
    let t3507 = t286 * t2887;
    let t3508 = t421 * t2844;
    let t3509 = t3508 * t2630;
    let t3510 = t3507 * t3509;
    let t3513 = t414 * t283;
    (t3497, t3499, t3500, t3501, t3502, t3504, t3505, t3509, t3510, t3513)
}
