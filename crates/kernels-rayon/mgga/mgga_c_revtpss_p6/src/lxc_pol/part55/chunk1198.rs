//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1198/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1198(t127370: f64, t2322: f64, t33581: f64, t4254: f64, t5517: f64, t651: f64, t8460: f64, t1868: f64, t7311: f64, t1459: f64, t34012: f64, t1916: f64, t32375: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t127371 = 2.0_f64 * t127370;
    let t127372 = t2322 * t33581;
    let t127373 = 2.0_f64 * t127372;
    let t127374 = t4254 * t33581;
    let t127375 = 2.0_f64 * t127374;
    let t127377 = t651 * t5517 * t8460;
    let t127378 = 2.0_f64 * t127377;
    let t127381 = t1868 * t7311;
    let t127453 = 6.0_f64 * t1459 * t34012;
    let t127455 = 6.0_f64 * t1916 * t32375;
    (t127371, t127373, t127375, t127378, t127381, t127453, t127455)
}
