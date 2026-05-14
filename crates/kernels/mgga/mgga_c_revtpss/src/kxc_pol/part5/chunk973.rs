//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 973/1286 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk973<F: Float>(t13032: F, t3599: F, t1242: F, t3603: F, t471: F, t3609: F, t3367: F, t414: F, t11239: F, t1243: F, t460: F, t3596: F, t1275: F, t225: F, t575: F, t5789: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13033 = t13032 * t3599;
    let t13037 = t1242 * t1242;
    let t13038 = 1.0 / t13037;
    let t13045 = t3603 * t471;
    let t13058 = t13032 * t3609;
    let t13099 = 1.0 / t414 / t3367;
    let t13126 = t11239 * t1243;
    let t13127 = t460 * t13126;
    let t13141 = t11239 * t3596;
    let t13142 = t460 * t13141;
    let t13147 = t11239 * t13038;
    let t13148 = t460 * t13147;
    let t13180 = t1275 * t1275;
    let t13181 = 1.0 / t13180;
    let t13182 = t225 * t13181;
    let t13254 = 2.0 * t5789 * t575;
    (t13033, t13045, t13058, t13099, t13126, t13127, t13141, t13142, t13147, t13148, t13182, t13254)
}
