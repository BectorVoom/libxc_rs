//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 720/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk720<F: Float>(t1347: F, t2001: F, t1352: F, t1998: F, t1446: F, t1423: F, t542: F, t7614: F, t537: F, t532: F, t7605: F, t1569: F, t1967: F, t2327: F, t7429: F, t7434: F, t7441: F, t7448: F, t7463: F) -> (F, F, F, F, F, F, F, F) {
    let t8704 = t2001 * t1347;
    let t8706 = t1998 * t1352;
    let t8708 = t2001 * t1446;
    let t8710 = t1998 * t1423;
    let t8712 = t7614 * t542;
    let t8714 = t7614 * t537;
    let t8716 = t7614 * t532;
    let t8718 = t7605 * t532;
    let t8720 = t2001 * t1569;
    let t8722 = t1967 * t2327;
    let t8728 = 0.34299214494455789578e-2 * t8704 - 0.85748036236139473944e-3 * t8706 - 0.34299214494455789578e-2 * t8708 + 0.17149607247227894789e-2 * t8710 + 0.40015750243531754507e-2 * t8712 - 0.40015750243531754507e-2 * t8714 + 0.80031500487063509015e-2 * t8716 - 0.17149607247227894789e-2 * t8718 - 0.17149607247227894789e-2 * t8720 - 0.64311027177104605458e-3 * t8722 - 0.47172138434406228102e-3 * t7429 - 0.94344276868812456204e-3 * t7434 - 0.28015625e-1 * t7441 - 0.420234375e-1 * t7448 - t7463;
    (t8706, t8710, t8712, t8714, t8716, t8718, t8722, t8728)
}
