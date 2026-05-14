//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 802/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk802<F: Float>(t2435: F, t3913: F, t133: F, t3880: F, t945: F, t2447: F, t1250: F, t2433: F, t2446: F, t3273: F, t3903: F, t397: F, t943: F, t942: F, t1246: F, t1256: F, t3904: F, t3910: F, t411: F, t415: F) -> (F, F, F, F, F, F, F) {
    let t3914 = t3913 * t2435;
    let t3919 = t3880 * t133;
    let t3920 = t3919 * t945;
    let t3923 = t3913 * t2447;
    let t3928 = 0.13170898365871023197e1 * t2433 * t3914 + 0.13170898365871023197e1 * t3273 * t1250 + 0.65854491829355115987e0 * t943 * t3920 - 0.65854491829355115987e0 * t2446 * t3923 + 0.65854491829355115987e0 * t397 * t3903;
    let t3929 = t942 * t3928;
    let t3932 = 0.65854491829355115987e0 * t3904 * t415 - 0.13170898365871023197e1 * t1246 * t1256 + 0.13170898365871023197e1 * t411 * t3910 - 0.65854491829355115987e0 * t411 * t3929;
    (t3914, t3919, t3920, t3923, t3928, t3929, t3932)
}
