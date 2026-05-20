//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1670/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1670<F: Float>(t3453: F, t5146: F, t3479: F, t5142: F, t1168: F, t3471: F, t12472: F, t1744: F, t1757: F, t3497: F, t1745: F, t1187: F, t5181: F) -> (F, F, F, F, F, F, F) {
    let t16955 = t5146 * t3453;
    let t16958 = t5142 * t3479;
    let t16959 = t16958 * t1168;
    let t16962 = t5146 * t3471;
    let t16965 = t1744 * t12472;
    let t16966 = t16965 * t3453;
    let t16971 = t1757 * t3497;
    let t16974 = t1745 * t3453;
    let t16979 = t5181 * t1187;
    (t16955, t16959, t16962, t16966, t16971, t16974, t16979)
}
