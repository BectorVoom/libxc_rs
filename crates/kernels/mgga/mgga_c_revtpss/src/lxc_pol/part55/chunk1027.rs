//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1027/1151 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1027<F: Float>(t122281: F, t1426: F, t7063: F, t7286: F, t32677: F, t686: F, t72: F, t32705: F, t32710: F, t136: F, t2457: F, t8708: F, t119971: F, t32275: F, t555: F, t32707: F, t94801: F) -> (F, F, F, F, F, F, F) {
    let t122282 = t122281 * t1426;
    let t122284 = t7063 * t122282 * t7286;
    let t122287 = t32677 * t72 * t686;
    let t122288 = t32705 * t122287;
    let t122290 = t32710 * t122287;
    let t122295 = t8708 * t136 * t2457;
    let t122297 = 0.6019057092162847523e-2 * t119971 * t555 * t32275 * t122295;
    let t122299 = t94801 * t32275 * t32707;
    (t122282, t122284, t122288, t122290, t122295, t122297, t122299)
}
