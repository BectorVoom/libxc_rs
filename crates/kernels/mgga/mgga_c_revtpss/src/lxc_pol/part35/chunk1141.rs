//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1141/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1141<F: Float>(t25898: F, t8099: F, t94849: F, t26277: F, t97916: F, t97799: F, t1358: F, t2439: F, t785: F, t8085: F, t136: F, t2457: F, t8094: F) -> (F, F, F, F, F) {
    let t102131 = t94849 * t25898 * t8099;
    let t102133 = t97916 * t26277;
    let t102135 = t97799 * t26277;
    let t102139 = t2439 * t785 * t8085 * t1358;
    let t102204 = t8094 * t136 * t2457;
    (t102131, t102133, t102135, t102139, t102204)
}
