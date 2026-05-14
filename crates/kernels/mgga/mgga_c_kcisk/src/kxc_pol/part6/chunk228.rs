//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 228/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk228<F: Float>(t10: F, t181: F, t179: F, t123: F, t15: F, t24: F, t151: F, t955: F, t180: F, t182: F, t183: F, t60: F, t852: F) -> (F, F, F, F, F, F) {
    let t983 = t10 * t181;
    let t987 = t179 * t179;
    let t988 = t987 * t987;
    let t989 = t988 * t179;
    let t990 = t123 * t989;
    let t991 = t24 * t15;
    let t995 = t151 * t955;
    let t1001 = 0.13140859333333333333e-2 * t180 * t983 * t183 - 0.98556444999999999995e-3 * t990 * t991 * t183 - 0.19711288999999999999e-2 * t180 * t182 * t995 - 4.0 * t60 * t852;
    (t983, t989, t990, t991, t995, t1001)
}
