//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 679/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk679<F: Float>(t1061: F, t72: F, t3117: F, t8502: F, t8504: F, t8508: F, t8509: F, t8514: F, t8517: F, t8522: F, t1989: F, t207: F, t8489: F, t8493: F, t198: F, t2411: F, t892: F) -> (F, F, F, F, F, F, F) {
    let t8523 = t1061 * t72;
    let t8524 = t8523 * t3117;
    let t8527 = 0.28234466758480466999e-3 * t8502 * t8504 - 0.8673628188205199462e0 * t8508 * t8509 + 0.57119737665102352616e0 * t8514 * t8517 - 0.1859366460452550541e-3 * t8522 * t8524;
    let t8531 = t1989 * t1989;
    let t8536 = t207 * t8489;
    let t8539 = t207 * t8493;
    let t8542 = -t198 * t2411 * t8539 + t198 * t8536 * t892;
    (t8523, t8524, t8527, t8531, t8536, t8539, t8542)
}
