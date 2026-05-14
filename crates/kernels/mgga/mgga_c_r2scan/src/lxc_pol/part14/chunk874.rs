//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 874/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk874<F: Float>(t2317: F, t502: F, t3446: F, t3448: F, t10949: F, t874: F, t3447: F, t2312: F, t3438: F, t1615: F, t6855: F, t166: F, t269: F, t1103: F, t1053: F, t10648: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10954 = t502 * t2317;
    let t10956 = t3446 * t10954 * t3448;
    let t10958 = t10949 * t874;
    let t10960 = t3446 * t3447 * t10958;
    let t10962 = t3438 * t2312;
    let t10964 = t3446 * t3447 * t10962;
    let t10966 = t6855 * t1615;
    let t10967 = t166 * t269;
    let t10968 = t1103 * t10967;
    let t10969 = t10966 * t10968;
    let t10971 = t1053 * t2317;
    let t10972 = t10971 * t3448;
    let t10973 = t10648 * t10972;
    (t10954, t10956, t10958, t10960, t10962, t10964, t10966, t10968, t10969, t10971, t10972, t10973)
}
