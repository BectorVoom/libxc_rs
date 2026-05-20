//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2262/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2262<F: Float>(t101453: F, t25823: F, t4287: F, t1513: F, t2340: F, t94982: F, t665: F, t25826: F, t2366: F, t13509: F, t6998: F, t101448: F, t101451: F, t94974: F, t94979: F, t94981: F) -> F {
    let t101454 = F::new(4.0) / F::new(3.0) * t101453;
    let t101455 = t25823 * t4287;
    let t101456 = F::new(2.0) / F::new(3.0) * t101455;
    let t101457 = t1513 * t2340;
    let t101458 = t94982 * t101457;
    let t101460 = t4287 * t665;
    let t101461 = t25826 * t101460;
    let t101463 = t1513 * t2366;
    let t101464 = t25826 * t101463;
    let t101466 = t6998 * t13509;
    let t101468 = -t94974 - t101448 - F::new(2.0) / F::new(3.0) * t94979 + t94981 / F::new(3.0) - F::new(11.0) / F::new(9.0) * t101451 - t101454 + t101456 - F::new(3.0) / F::new(4.0) * t101458 + t101461 / F::new(2.0) + t101464 / F::new(4.0) - t101466 / F::new(8.0);
    t101468
}
