//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1074/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1074<F: Float>(t1422: F, t1512: F, t3: F, t5243: F, t273: F, t625: F, t409: F, t4732: F, t1266: F, t1391: F, t1390: F, t2090: F, t386: F, t385: F, t18783: F, t5: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18942 = t1422 * t1512;
    let t18944 = t5243 * t3;
    let t18946 = t18944 * t273 * t625;
    let t18948 = t4732 * t409;
    let t18950 = t1391 * t1266;
    let t18951 = t1390 * t18950;
    let t18953 = t386 * t2090;
    let t18954 = t385 * t18953;
    let t18956 = t5 * t18783;
    (t18942, t18944, t18946, t18948, t18950, t18951, t18953, t18954, t18956)
}
