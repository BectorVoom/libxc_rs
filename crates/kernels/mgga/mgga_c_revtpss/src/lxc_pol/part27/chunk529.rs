//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 529/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk529<F: Float>(t1043: F, t999: F, t1045: F, t3117: F, t1032: F, t989: F, t1040: F, t1024: F, t1062: F, t1065: F, t906: F, t1042: F, t2868: F, t2871: F, t2878: F, t2921: F, t2929: F, t3019: F, t3021: F, t3024: F, t3028: F, t3032: F, t3036: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3118 = t999 * t1043;
    let t3119 = t3118 * t1045;
    let t3120 = t3117 * t3119;
    let t3123 = t989 * t1032;
    let t3124 = t3123 * t1040;
    let t3127 = t1024 * t1062;
    let t3128 = t1065 * t999;
    let t3129 = t3128 * t906;
    let t3130 = t1042 * t3129;
    let t3133 = -t2868 + t2871 - t2878 + t2921 + t2929 + t3019 + t3021 - t3024 + t3028 - t3032 - t3036;
    (t3118, t3119, t3120, t3123, t3124, t3127, t3128, t3129, t3130, t3133)
}
