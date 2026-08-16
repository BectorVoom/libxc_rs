//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1545/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1545<F: Float>(t4757: F, t5004: F, t3291: F, t6244: F, t1082: F, t19399: F, t4866: F, t4982: F, t4893: F, t1647: F, t4980: F, t1071: F, t6305: F) -> (F, F, F, F, F, F, F) {
    let t19509 = t5004 * t4757;
    let t19512 = t3291 * t6244;
    let t19515 = t1082 * t19399;
    let t19520 = t4982 * t4866;
    let t19521 = t4893 * t19520;
    let t19526 = t1647 * t4980;
    let t19533 = t1071 * t6305;
    (t19509, t19512, t19515, t19520, t19521, t19526, t19533)
}
