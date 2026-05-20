//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1155/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1155<F: Float>(t121940: F, t25374: F, t25378: F, t122002: F, t2097: F, t7063: F, t25877: F, t25881: F, t786: F, t1032: F, t7506: F, t1426: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t122034 = t121940 * t25374;
    let t122035 = t122034 * t25378;
    let t122037 = t122002 * t25374;
    let t122038 = t122037 * t25378;
    let t122273 = t7063 * t2097;
    let t122274 = t122273 * t25877;
    let t122275 = t122274 * t25881;
    let t122277 = t786 * t2097;
    let t122278 = t122277 * t25877;
    let t122279 = t122278 * t25881;
    let t122281 = t7506 * t1032;
    let t122282 = t122281 * t1426;
    (t122034, t122035, t122037, t122038, t122273, t122274, t122275, t122277, t122278, t122279, t122281, t122282)
}
