//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1194/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1194(t18227: f64, t8461: f64, t32110: f64, t4248: f64, t2322: f64, t33581: f64, t4254: f64, t5517: f64, t651: f64, t8460: f64, t1868: f64, t7311: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127368 = t18227 * t8461;
    let t127370 = t4248 * t32110;
    let t127372 = t2322 * t33581;
    let t127374 = t4254 * t33581;
    let t127377 = t651 * t5517 * t8460;
    let t127381 = t1868 * t7311;
    (t127368, t127370, t127372, t127374, t127377, t127381)
}
