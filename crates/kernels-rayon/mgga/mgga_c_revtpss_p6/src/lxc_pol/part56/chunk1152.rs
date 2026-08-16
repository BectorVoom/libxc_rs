//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1152/1203 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1152(t127370: f64, t2322: f64, t33581: f64, t4254: f64, t5517: f64, t651: f64, t8460: f64, t1868: f64, t7311: f64, t25082: f64, t8717: f64, t27833: f64, t8600: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t127371 = 2.0_f64 * t127370;
    let t127372 = t2322 * t33581;
    let t127373 = 2.0_f64 * t127372;
    let t127374 = t4254 * t33581;
    let t127375 = 2.0_f64 * t127374;
    let t127377 = t651 * t5517 * t8460;
    let t127378 = 2.0_f64 * t127377;
    let t127381 = t1868 * t7311;
    let t127384 = 6.0_f64 * t25082 * t8717 * t127381;
    let t127385 = t27833 * t8600;
    (t127371, t127373, t127375, t127378, t127384, t127385)
}
