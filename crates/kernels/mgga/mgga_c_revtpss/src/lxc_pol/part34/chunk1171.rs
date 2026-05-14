//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1171/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1171<F: Float>(t114: F, t30004: F, t4248: F, t22633: F, t93: F, t1936: F, t30143: F, t7741: F, t7889: F, t22589: F, t94982: F, t25826: F, t75833: F, t22628: F, t6998: F, t101451: F, t105870: F, t105878: F, t94974: F) -> (F, F, F, F, F) {
    let t115 = 1.0 < t114;
    let t114384 = 6.0 * t4248 * t30004;
    let t114385 = t93 * t22633;
    let t114387 = 2.0 * t114385 * t1936;
    let t114389 = 6.0 * t30143 * t7741;
    let t114391 = 6.0 * t7889 * t30004;
    let t114394 = t94982 * t22589;
    let t114396 = t25826 * t75833;
    let t114398 = t6998 * t22628;
    let t114401 = piecewise3(t115, 0.0, -t94974 - 11.0 / 3.0 * t101451 - 2.0 * t105870 + t105878 - 3.0 / 4.0 * t114394 + 3.0 / 4.0 * t114396 - t114398 / 8.0);
    (t114384, t114387, t114389, t114391, t114401)
}
