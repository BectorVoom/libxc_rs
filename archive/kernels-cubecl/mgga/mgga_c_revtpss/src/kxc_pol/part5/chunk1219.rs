//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1219/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1219<F: Float>(t1079: F, t1651: F, t5015: F, t4772: F, t996: F, t16313: F, t4940: F, t6258: F, t999: F, t1096: F, t6244: F, t6350: F) -> (F, F, F, F, F, F, F, F) {
    let t19396 = t1079 * t1651 * t5015;
    let t19399 = t1651 * t4772;
    let t19400 = t996 * t19399;
    let t19403 = t16313 * t4940;
    let t19414 = t6258 * t999;
    let t19415 = t996 * t19414;
    let t19421 = t1079 * t6244 * t1096;
    let t19424 = t6350 * t1096;
    (t19396, t19399, t19400, t19403, t19414, t19415, t19421, t19424)
}
