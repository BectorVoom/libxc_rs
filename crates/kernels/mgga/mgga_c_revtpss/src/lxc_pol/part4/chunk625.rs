//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 625/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk625<F: Float>(t1065: F, t999: F, t906: F, t1042: F, t2868: F, t2871: F, t2878: F, t2921: F, t2929: F, t3019: F, t3021: F, t3024: F, t3028: F, t3032: F, t3036: F) -> (F, F, F) {
    let t3128 = t1065 * t999;
    let t3129 = t3128 * t906;
    let t3130 = t1042 * t3129;
    let t3133 = -t2868 + t2871 - t2878 + t2921 + t2929 + t3019 + t3021 - t3024 + t3028 - t3032 - t3036;
    (t3129, t3130, t3133)
}
