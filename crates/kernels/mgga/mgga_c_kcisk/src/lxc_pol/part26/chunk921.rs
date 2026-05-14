//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 921/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk921<F: Float>(t1186: F, t25469: F, t1398: F, t25432: F, t1375: F, t25437: F, t25465: F, t457: F, t8123: F, t970: F, t8126: F, t960: F, t8139: F, t8142: F, t8145: F, t8148: F, t965: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t25470 = t1186 * t25469;
    let t25473 = t1398 * t25432;
    let t25476 = t1375 * t25437;
    let t25479 = t457 * t25465;
    let t25482 = t1375 * t25469;
    let t25485 = t970 * t8123;
    let t25487 = t960 * t8126;
    let t25489 = t970 * t8139;
    let t25491 = t970 * t8142;
    let t25493 = t960 * t8145;
    let t25495 = t965 * t8148;
    (t25470, t25473, t25476, t25479, t25482, t25485, t25487, t25489, t25491, t25493, t25495)
}
