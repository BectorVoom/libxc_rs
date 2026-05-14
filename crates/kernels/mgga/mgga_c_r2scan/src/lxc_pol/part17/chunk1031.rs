//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1031/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1031<F: Float>(t1592: F, t29270: F, t3308: F, t2196: F, t29274: F, t1054: F, t5108: F, t8760: F, t6583: F, t8764: F, t8769: F, t10894: F, t3086: F, t30285: F, t3332: F, t6165: F) -> (F, F, F, F, F, F, F) {
    let t43248 = t1592 * t3308 * t29270;
    let t43251 = t2196 * t3308 * t29274;
    let t43256 = t5108 * t1054 * t8760;
    let t43259 = t6583 * t1054 * t8764;
    let t43262 = t5108 * t1054 * t8769;
    let t43266 = t10894 * t3086;
    let t43269 = t6165 * t3332 * t30285;
    (t43248, t43251, t43256, t43259, t43262, t43266, t43269)
}
