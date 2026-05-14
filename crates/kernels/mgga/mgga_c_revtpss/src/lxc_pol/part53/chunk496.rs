//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 496/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk496<F: Float>(t114: F, t108: F, t2: F, t580: F, t105: F, t1505: F, t1507: F, t4270: F, t4274: F, t4280: F, t656: F, t662: F, t97: F, t655: F, t2335: F, t2336: F, t4261: F, t4264: F, t69: F) -> (F, F) {
    let t115 = 1.0 < t114;
    let t4283 = t108 * t2;
    let t4284 = t4283 * t580;
    let t4287 = -25.0 / 9.0 * t656 * t1505 + 10.0 / 9.0 * t97 * t4270 + 5.0 / 3.0 * t97 * t4274 - 25.0 / 9.0 * t1507 * t662 + 10.0 / 9.0 * t105 * t4280 - 5.0 / 3.0 * t105 * t4284;
    let t4288 = t655 * t4287;
    let t4292 = piecewise3(t115, 0.0, t2335 + t2336 / 3.0 + t4261 / 3.0 + t69 * t4264 / 4.0 - t69 * t4288 / 8.0);
    (t4287, t4292)
}
