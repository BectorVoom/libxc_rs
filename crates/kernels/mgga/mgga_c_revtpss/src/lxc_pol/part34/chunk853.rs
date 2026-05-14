//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 853/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk853<F: Float>(t4269: F, t5823: F, t580: F, t9342: F, t100: F, t5842: F, t1509: F, t5907: F, t10241: F, t4279: F, t5911: F, t108: F, t105: F, t109: F, t1507: F, t1510: F, t22597: F, t5902: F, t5908: F, t5912: F, t97: F, tau1: F) -> (F, F) {
    let t22600 = t4269 * t5823;
    let t22603 = -t580 - t9342;
    let t22604 = 3.0 * t22603;
    let t22605 = t100 * t22604;
    let t22608 = tau1 * t5842;
    let t22617 = t5907 * t1509;
    let t22618 = t10241 * t22617;
    let t22621 = t4279 * t5911;
    let t22624 = -t22604;
    let t22625 = t108 * t22624;
    let t22628 = -10.0 / 27.0 * t97 * t22597 + 10.0 / 3.0 * t97 * t22600 + 5.0 / 3.0 * t97 * t22605 - 440.0 / 27.0 * t22608 * t109 + 200.0 / 9.0 * t5902 * t1510 - 50.0 / 9.0 * t1507 * t5908 - 25.0 / 3.0 * t1507 * t5912 - 10.0 / 27.0 * t105 * t22618 + 10.0 / 3.0 * t105 * t22621 + 5.0 / 3.0 * t105 * t22625;
    (t22603, t22628)
}
