//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1941/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1941<F: Float>(t1513: F, t94975: F, t28036: F, t94978: F, t25823: F, t4287: F, t2340: F, t94982: F, t665: F, t25826: F, t2366: F, t13509: F, t6998: F) -> (F, F, F, F, F, F, F) {
    let t101451 = t94975 * t1513;
    let t101453 = t94978 * t28036;
    let t101455 = t25823 * t4287;
    let t101457 = t1513 * t2340;
    let t101458 = t94982 * t101457;
    let t101460 = t4287 * t665;
    let t101461 = t25826 * t101460;
    let t101463 = t1513 * t2366;
    let t101464 = t25826 * t101463;
    let t101466 = t6998 * t13509;
    (t101451, t101453, t101455, t101458, t101461, t101464, t101466)
}
