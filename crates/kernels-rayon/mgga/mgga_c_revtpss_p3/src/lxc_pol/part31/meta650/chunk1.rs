//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2145/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2145(t19658: f64, t7122: f64, t19920: f64, t25522: f64, t27489: f64, t4817: f64, t100002: f64, t100006: f64, t100025: f64, t100114: f64, t1675: f64, t19677: f64, t19895: f64, t20083: f64, t25569: f64, t27536: f64, t4912: f64, t6263: f64, t6331: f64, t93646: f64) -> (f64, f64) {
    let t106877 = t7122 * t19658;
    let t106896 = t25522 * t19920;
    let t106906 = t27489 * t4817;
    let t106913 = 0.30488190661738479625e-2_f64 * t93646 * t6263 - 0.38110238327173099531e-3_f64 * t106896 - 0.28582678745379824648e-3_f64 * t25522 * t19677 + 0.57165357490759649296e-3_f64 * t25522 * t19895 - 0.57165357490759649296e-3_f64 * t25569 * t6331 - 0.30488190661738479625e-2_f64 * t100114 * t1675 + 0.38110238327173099531e-3_f64 * t106906 + 0.85748036236139473944e-3_f64 * t27536 * t20083 - 0.19055119163586549765e-3_f64 * t100002 + t100006 - 0.85748036236139473944e-3_f64 * t100025 * t4912;
    (t106877, t106913)
}
