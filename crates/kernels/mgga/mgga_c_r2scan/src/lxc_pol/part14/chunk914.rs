//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 914/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk914<F: Float>(t12029: F, t3275: F, t11336: F, t3270: F, t986: F, t3269: F, t11325: F, t3582: F, t1044: F, t3560: F, t11345: F, t3579: F, t11625: F, t3465: F, t11475: F, t3262: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12030 = t3275 * t12029;
    let t12031 = 5.0 / 16.0 * t12030;
    let t12033 = t3270 * t11336 * t986;
    let t12034 = t3269 * t12033;
    let t12035 = t12034 / 4.0;
    let t12037 = t3275 * t11325 * t3582;
    let t12038 = 5.0 / 16.0 * t12037;
    let t12039 = t3560 * t1044;
    let t12040 = t3579 * t11345;
    let t12041 = t12040 / 4.0;
    let t12042 = t3465 * t11625;
    let t12043 = t3275 * t12042;
    let t12044 = t12043 / 2.0;
    let t12045 = t3465 * t11475;
    let t12046 = t3262 * t12045;
    (t12030, t12031, t12033, t12034, t12035, t12037, t12038, t12039, t12040, t12041, t12042, t12043, t12044, t12045, t12046)
}
