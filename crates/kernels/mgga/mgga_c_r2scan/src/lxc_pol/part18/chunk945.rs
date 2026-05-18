//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 945/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk945<F: Float>(t10962: F, t3446: F, t3447: F, t1615: F, t6855: F, t166: F, t269: F, t1103: F, t1053: F, t2317: F, t3448: F, t10648: F) -> (F, F, F, F, F, F, F) {
    let t10964 = t3446 * t3447 * t10962;
    let t10965 = F::new(0.15243824895787514157e-3) * t10964;
    let t10966 = t6855 * t1615;
    let t10967 = t166 * t269;
    let t10968 = t1103 * t10967;
    let t10969 = t10966 * t10968;
    let t10970 = F::new(0.34200192530023447503e-6) * t10969;
    let t10971 = t1053 * t2317;
    let t10972 = t10971 * t3448;
    let t10973 = t10648 * t10972;
    (t10965, t10966, t10968, t10970, t10971, t10972, t10973)
}
