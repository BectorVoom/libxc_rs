//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 882/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk882<F: Float>(t10967: F, t1103: F, t10966: F, t1053: F, t2317: F, t3448: F, t10648: F, t10933: F, t10937: F, t10942: F, t10945: F, t10948: F, t10952: F, t10957: F, t10960: F, t10965: F) -> (F, F, F, F, F, F) {
    let t10968 = t1103 * t10967;
    let t10969 = t10966 * t10968;
    let t10970 = 0.34200192530023447503e-6 * t10969;
    let t10971 = t1053 * t2317;
    let t10972 = t10971 * t3448;
    let t10973 = t10648 * t10972;
    let t10974 = 0.30487649791575028314e-3 * t10973;
    let t10975 = -t10933 + 0.19211284388664477842e-2 * t10937 - t10942 + t10945 + t10948 + 0.43368970657079495312e-4 * t10952 + t10957 - 0.30487649791575028314e-3 * t10960 - t10965 + t10970 + t10974;
    (t10968, t10970, t10971, t10972, t10974, t10975)
}
