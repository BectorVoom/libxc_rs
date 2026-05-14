//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 990/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk990<F: Float>(t22965: F, t6674: F, t10414: F, t8486: F, t220: F, t2441: F, t5193: F, t5192: F, t15903: F, t6690: F, t6974: F, t1869: F, t6698: F, t6719: F, t1799: F, t1636: F, t9029: F) -> (F, F, F, F, F, F, F) {
    let t22966 = t6674 * t22965;
    let t22968 = t10414 * t8486;
    let t22970 = t220 * t2441;
    let t22971 = t5193 * t22970;
    let t22972 = t5192 * t22971;
    let t22973 = t15903 * t22972;
    let t22976 = t6974 * t6690;
    let t22977 = t1869 * t22976;
    let t22979 = t6719 * t6698;
    let t22980 = t1799 * t22979;
    let t22984 = t9029 * t1636;
    (t22966, t22968, t22971, t22973, t22977, t22980, t22984)
}
