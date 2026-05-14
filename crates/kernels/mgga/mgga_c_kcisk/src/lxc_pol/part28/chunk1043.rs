//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1043/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1043<F: Float>(t1869: F, t23982: F, t1060: F, t2527: F, t6763: F, t17044: F, t5182: F, t6758: F, t6674: F, t6689: F, t6965: F, t1873: F, t4817: F, t8947: F, t2441: F, t7069: F) -> (F, F, F, F, F, F, F, F) {
    let t23983 = t1869 * t23982;
    let t23985 = t2527 * t1060;
    let t23986 = t6763 * t23985;
    let t23987 = t17044 * t23986;
    let t23988 = t5182 * t23987;
    let t23990 = t6758 * t23985;
    let t23991 = t17044 * t23990;
    let t23992 = t6674 * t23991;
    let t23994 = t6965 * t6689;
    let t23995 = t1873 * t23994;
    let t23996 = t1869 * t23995;
    let t23998 = t4817 * t8947;
    let t23999 = t1869 * t23998;
    let t24001 = t7069 * t2441;
    (t23983, t23986, t23988, t23990, t23992, t23996, t23999, t24001)
}
