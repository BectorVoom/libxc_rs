//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 881/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk881<F: Float>(t10950: F, t3434: F, t3437: F, t2317: F, t502: F, t3446: F, t3448: F, t10949: F, t874: F, t3447: F, t2312: F, t3438: F, t1615: F, t6855: F, t166: F, t269: F) -> (F, F, F, F, F, F, F, F, F) {
    let t10952 = t3434 * t3437 * t10950;
    let t10954 = t502 * t2317;
    let t10956 = t3446 * t10954 * t3448;
    let t10957 = 0.81300399444200075504e-3 * t10956;
    let t10958 = t10949 * t874;
    let t10960 = t3446 * t3447 * t10958;
    let t10962 = t3438 * t2312;
    let t10964 = t3446 * t3447 * t10962;
    let t10965 = 0.15243824895787514157e-3 * t10964;
    let t10966 = t6855 * t1615;
    let t10967 = t166 * t269;
    (t10952, t10954, t10957, t10958, t10960, t10962, t10965, t10966, t10967)
}
