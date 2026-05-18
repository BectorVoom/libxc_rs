//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1048/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1048<F: Float>(t1089: F, t23992: F, t23997: F, t24007: F, t3304: F, t3318: F, t5004: F, t6244: F, t1082: F, t24031: F, t24111: F, t23598: F) -> (F, F, F, F, F, F, F, F) {
    let t24132 = t23992 * t1089;
    let t24135 = t23997 * t1089;
    let t24138 = t24007 * t3304;
    let t24141 = t24007 * t3318;
    let t24144 = t5004 * t6244;
    let t24147 = t1082 * t24031;
    let t24152 = t24111 * t3318;
    let t24157 = t1082 * t23598;
    (t24132, t24135, t24138, t24141, t24144, t24147, t24152, t24157)
}
