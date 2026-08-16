//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1281/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1281(t2322: f64, t34025: f64, t4254: f64, t651: f64, t7474: f64, t7741: f64, t34167: f64, t670: f64, t127365: f64, t127368: f64, t127370: f64, t127372: f64, t127374: f64, t127377: f64, t28696: f64, t34279: f64, t6985: f64, t7221: f64, t7983: f64) -> f64 {
    let t128998 = t2322 * t34025;
    let t128999 = t4254 * t34025;
    let t129001 = t651 * t7474 * t7741;
    let t129008 = t651 * t34167 * t670;
    let t129009 = -t651 * t7221 * t7983 - t2322 * t34279 - t28696 * t6985 - t34279 * t4254 - t127365 - t127368 - t127370 - t127372 - t127374 - t127377 - t128998 - t128999 - t129001 - t129008;
    t129009
}
