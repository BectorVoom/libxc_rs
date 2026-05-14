//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1371/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1371<F: Float>(t1339: F, t18954: F, t9461: F, t1308: F, t388: F, t54621: F, t1220: F, t6147: F, t19972: F, t3930: F, t109613: F, t9814: F, t110319: F, t113636: F, t113783: F, t32026: F, t32066: F, t32127: F, t33373: F, t33389: F, t33400: F, t9426: F, t9429: F, t9454: F, t9796: F) -> (F, F, F, F, F) {
    let t114188 = t1339 * t9461 * t18954;
    let t114195 = t54621 * t388 * t1308;
    let t114199 = t1220 * t6147 * t1308;
    let t114205 = t19972 * t388 * t1308;
    let t114209 = t3930 * t6147 * t1308;
    let t114215 = t1339 * t109613 * t9814;
    let t114217 = -0.8041666666666666667e-2 * t32026 * t33400 - 0.8041666666666666667e-2 * t32066 * t33400 - 0.8041666666666666667e-2 * t9426 * t113783 - 0.40208333333333333335e-2 * t9426 * t113636 - 0.24125000000000000001e-1 * t32026 * t33389 - 0.33163888888888888888e-2 * t114188 - 0.34722222222222222223e-2 * t33373 * t32127 + 0.39314814814814814818e-1 * t110319 * t9796 + 0.8041666666666666667e-2 * t114195 * t9429 + 0.20833333333333333334e-1 * t114199 * t9454 + 0.20833333333333333334e-1 * t114199 * t9429 + 0.20833333333333333334e-1 * t114205 * t9454 + 0.8041666666666666667e-2 * t114209 * t9429 + 0.20833333333333333334e-1 * t114205 * t9429 + 0.16581944444444444444e-2 * t114215;
    (t114188, t114199, t114205, t114215, t114217)
}
