//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1172/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1172<F: Float>(t3057: F, t379: F, t1078: F, t1651: F, t3066: F, t1695: F, t3325: F, t3269: F, t3270: F, t11121: F, t5015: F, t999: F, t1079: F, t342: F, t4930: F, t1071: F, t1647: F) -> (F, F, F, F, F, F, F) {
    let t16312 = t3057 * t379;
    let t16313 = t1078 * t1651;
    let t16314 = t16313 * t3066;
    let t16317 = t1695 * t3325;
    let t16318 = t3269 * t16317;
    let t16321 = t1695 * t3270;
    let t16322 = t11121 * t16321;
    let t16327 = t5015 * t999;
    let t16328 = t1079 * t16327;
    let t16333 = t342 * t4930;
    let t16340 = t1647 * t1071;
    (t16312, t16314, t16318, t16322, t16328, t16333, t16340)
}
