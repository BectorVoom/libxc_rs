//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2661/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2661<F: Float>(t1647: F, t16565: F, t12166: F, t1678: F, t342: F, t12077: F, t20050: F, t3106: F, t1063: F, t247: F, t42447: F, t6092: F) -> (F, F, F, F, F) {
    let t65181 = t1647 * t16565;
    let t65216 = t342 * t12166 * t1678;
    let t65220 = t342 * t12077 * t1678;
    let t65288 = t3106 * t20050;
    let t65292 = t1063 * t247 * t42447 * t6092;
    (t65181, t65216, t65220, t65288, t65292)
}
