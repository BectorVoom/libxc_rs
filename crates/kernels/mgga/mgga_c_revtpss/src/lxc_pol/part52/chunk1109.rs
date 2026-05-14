//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1109/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1109<F: Float>(t2322: F, t34028: F, t128528: F, t128531: F, t128533: F, t128535: F, t128537: F, t128539: F, t128543: F, t128552: F, t13426: F, t18227: F, t28053: F, t32410: F, t4248: F, t7359: F, t8637: F) -> (F,) {
    let t128554 = 2.0 * t2322 * t34028;
    let t128555 = -2.0 * t13426 * t8637 - 2.0 * t18227 * t8637 - 2.0 * t28053 * t7359 - 2.0 * t32410 * t4248 + t128528 + t128531 - t128533 - t128535 - t128537 - t128539 - t128543 - t128552 - t128554;
    (t128555,)
}
