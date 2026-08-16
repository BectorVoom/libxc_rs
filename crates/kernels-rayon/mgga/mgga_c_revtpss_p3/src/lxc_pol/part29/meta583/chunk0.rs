//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1935/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1935(t14468: f64, t33: f64, t25759: f64, t61102: f64, t61182: f64, t27799: f64, t98779: f64, t1711: f64, t2394: f64, t2430: f64, t27375: f64, t94245: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t101051 = t33 * t14468;
    let t101055 = t25759 * t61102;
    let t101061 = t25759 * t61182;
    let t101065 = t27799 * t98779;
    let t101070 = t1711 * t2394;
    let t101074 = t1711 * t2430;
    let t101083 = t94245 * t27375;
    (t101051, t101055, t101061, t101065, t101070, t101074, t101083)
}
