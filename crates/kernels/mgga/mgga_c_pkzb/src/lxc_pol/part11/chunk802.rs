//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 802/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk802<F: Float>(t8919: F, t8944: F, t8993: F, t9017: F, t158: F, t3466: F, t5418: F, t633: F, t2678: F, t2702: F, t1790: F, t3487: F, t3410: F, t621: F, t1044: F, t1717: F) -> (F, F, F, F, F, F, F, F) {
    let t9019 = t8919 + t8944 + t8993 + t9017;
    let t9020 = t9019 * t158;
    let t9033 = t5418 * t3466;
    let t9034 = t9033 * t633;
    let t9037 = t2678 * t2702;
    let t9042 = t1790 * t3487;
    let t9043 = t9042 * t633;
    let t9048 = t621 * t3410;
    let t9056 = t1717 * t1044;
    (t9019, t9020, t9033, t9034, t9037, t9043, t9048, t9056)
}
