//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1051/1292 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1051(t27383: f64, t32505: f64, t1962: f64, t605: f64, t1940: f64, t198: f64, t207: f64, t2403: f64, t26585: f64, t26590: f64, t32486: f64, t32491: f64, t32498: f64, t7086: f64, t7432: f64, t775: f64, t8657: f64, t890: f64, t892: f64) -> (f64, f64, f64) {
    let t32506 = t27383 * t32505;
    let t32508 = t605 * t1962;
    let t32534 = t198 * t207 * t32486 * t892 - t1940 * t1962 * t26585 + 2.0_f64 * t1940 * t26590 * t32505 - t1940 * t32491 * t890 - t1940 * t7086 * t7432 - 3.0_f64 * t2403 * t32498 * t7432 + 3.0_f64 * t2403 * t775 * t8657;
    (t32506, t32508, t32534)
}
