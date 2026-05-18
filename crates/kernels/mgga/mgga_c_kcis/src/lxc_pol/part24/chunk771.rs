//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 771/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk771<F: Float>(t1071: F, t1083: F, t2844: F, t1160: F, t318: F, t86: F, t284: F, t3473: F, t3177: F, t3436: F, t1194: F, t381: F) -> (F, F, F, F, F, F) {
    let t10560 = t1083 * t1071;
    let t10583 = t1083 * t2844;
    let t10631 = t86 * t318 * t1160;
    let t10707 = t3473 * t284;
    let t10745 = t3177 * t3436;
    let t10752 = t381 * t1194;
    (t10560, t10583, t10631, t10707, t10745, t10752)
}
