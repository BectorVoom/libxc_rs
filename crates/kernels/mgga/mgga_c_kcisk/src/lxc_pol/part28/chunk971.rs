//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 971/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk971<F: Float>(t1646: F, t8533: F, t10671: F, t8504: F, t1648: F, t6771: F, t6777: F, t4663: F, t8522: F, t22596: F, t22564: F, t22567: F, t22570: F, t22573: F, t22575: F, t22578: F, t22581: F, t22583: F, t22586: F, t22589: F, t22594: F) -> (F, F, F, F, F, F) {
    let t22599 = t1646 * t8533;
    let t22602 = t10671 * t8504;
    let t22603 = t22602 * t1648;
    let t22605 = t6777 * t6771;
    let t22607 = t4663 * t8522;
    let t22608 = t22607 * t1648;
    let t22610 = t1646 * t22596;
    let t22628 = 0.91722222222222222223e-3 * t22564 - 0.45861111111111111112e-2 * t22567 + 0.1651e-1 * t22570 + 0.11006666666666666667e-1 * t22573 - 0.27516666666666666667e-2 * t22575 - 0.24765e-1 * t22578 - 0.3302e-1 * t22581 + 0.13758333333333333333e-2 * t22583 - 0.27516666666666666667e-2 * t22586 + 0.8255e-2 * t22589 - 0.41275e-2 * t22594;
    (t22599, t22603, t22605, t22608, t22610, t22628)
}
