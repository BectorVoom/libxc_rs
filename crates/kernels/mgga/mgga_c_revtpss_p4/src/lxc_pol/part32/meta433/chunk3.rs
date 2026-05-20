//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1544/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1544<F: Float>(t1248: F, t13045: F, t20956: F, t3720: F, t5341: F, t1219: F, t6667: F, t247: F, t3634: F, t6429: F, t1261: F, t12856: F, t20795: F) -> (F, F, F, F, F) {
    let t20957 = t13045 * t1248;
    let t20958 = t20956 * t20957;
    let t20959 = t3720 * t20958;
    let t20962 = t20956 * t5341;
    let t20963 = t3720 * t20962;
    let t20966 = t6667 * t1219;
    let t20973 = t247 * t3634 * t6429;
    let t20974 = t1261 * t20973;
    let t20977 = t20795 * t12856;
    (t20959, t20963, t20966, t20974, t20977)
}
