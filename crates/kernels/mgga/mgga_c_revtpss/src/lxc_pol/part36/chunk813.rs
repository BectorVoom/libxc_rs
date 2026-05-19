//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 813/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk813<F: Float>(t235: F, t9731: F, t1389: F, t3964: F, t2735: F, t546: F, t1369: F, t2699: F, t3943: F, t794: F, t1412: F, t159: F) -> (F, F, F, F, F, F) {
    let t9732 = t235 * t9731;
    let t9735 = F::cast_from(0.81322168495418382223e-4_f64) * t3964 * t9732 * t1389;
    let t9736 = t2735 * t546;
    let t9741 = t2699 * t1369;
    let t9744 = t794 * t3943;
    let t9747 = t159 * t1412;
    (t9732, t9735, t9736, t9741, t9744, t9747)
}
