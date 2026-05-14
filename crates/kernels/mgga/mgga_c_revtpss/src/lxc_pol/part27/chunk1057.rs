//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1057/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1057<F: Float>(t2258: F, t77: F, t84: F, t10327: F, t603: F, t10310: F, t2248: F, t640: F, t10298: F, t607: F, t2242: F, t2259: F, t25856: F, t4254: F, t13207: F, t1936: F, t651: F) -> (F, F, F, F, F, F, F, F) {
    let t92672 = t77 * t84 * t2258;
    let t92674 = t603 * t10327;
    let t92692 = t77 * t84 * t10310;
    let t92696 = t77 * t640 * t2248;
    let t92709 = t10298 * t607;
    let t92711 = t2242 * t2259;
    let t92724 = 6.0 * t4254 * t25856;
    let t92727 = 2.0 * t651 * t13207 * t1936;
    (t92672, t92674, t92692, t92696, t92709, t92711, t92724, t92727)
}
