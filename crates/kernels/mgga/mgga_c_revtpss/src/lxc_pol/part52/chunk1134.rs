//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1134/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1134<F: Float>(t2322: F, t34025: F, t4254: F, t651: F, t7474: F, t7741: F, t34167: F, t670: F, t127365: F, t127368: F, t127370: F, t127372: F, t127374: F, t127377: F, t28696: F, t34279: F, t6985: F, t7221: F, t7983: F) -> (F,) {
    let t128998 = t2322 * t34025;
    let t128999 = t4254 * t34025;
    let t129001 = t651 * t7474 * t7741;
    let t129008 = t651 * t34167 * t670;
    let t129009 = -t651 * t7221 * t7983 - t2322 * t34279 - t28696 * t6985 - t34279 * t4254 - t127365 - t127368 - t127370 - t127372 - t127374 - t127377 - t128998 - t128999 - t129001 - t129008;
    (t129009,)
}
