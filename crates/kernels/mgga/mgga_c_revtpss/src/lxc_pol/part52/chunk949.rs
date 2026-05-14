//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 949/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk949<F: Float>(t5: F, t32597: F, t8623: F, t1925: F, t84: F, t640: F, t8621: F, t32151: F, t32581: F, t32584: F, t32586: F, t32590: F, t32593: F, t8620: F, t117: F, t1310: F, t2322: F, t32402: F, t32404: F, t32410: F, t32415: F, t32417: F, t32419: F, t32421: F, t32576: F, t32580: F, t4254: F, t508: F, t651: F, t6985: F, t7378: F, t8627: F, t8637: F) -> (F, F, F, F, F, F) {
    let t7 = piecewise3(0.0 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0;
    let t32599 = 5.0 / 27.0 * t32597 * t8623;
    let t32600 = t84 * t1925;
    let t32602 = t8621 * t32600 * t640;
    let t32608 = piecewise3(t8, 0.0, -5.0 / 72.0 * t32581 * t8623 + 5.0 / 12.0 * t32584 * t32586 + 5.0 / 18.0 * t32590 * t32593 + t32599 - 5.0 / 36.0 * t8620 * t32602 - 5.0 / 72.0 * t8620 * t32151);
    let t32609 = t32608 * t117;
    let t32612 = -t1310 * t8627 - 2.0 * t2322 * t8637 - 2.0 * t32410 * t651 - t32609 * t508 - 2.0 * t4254 * t8637 - 2.0 * t6985 * t7378 - 2.0 * t32402 - 2.0 * t32404 - t32415 - t32417 - t32419 - t32421 - t32576 + t32580;
    (t32599, t32600, t32602, t32608, t32609, t32612)
}
