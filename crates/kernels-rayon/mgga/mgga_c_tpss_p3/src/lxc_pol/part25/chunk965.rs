//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 965/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk965(t1334: f64, t2023: f64, t3509: f64, t600: f64, t3533: f64, t2083: f64, t97: f64, t105: f64, t2091: f64, t10281: f64, t10282: f64, t10283: f64, t10284: f64, t10285: f64, t10286: f64, t7656: f64, t7659: f64, t7665: f64, t7668: f64, t7676: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t13154 = t2023 * t1334;
    let t13157 = 4.0_f64 / 3.0_f64 * t600 * t3509;
    let t13159 = 2.0_f64 / 3.0_f64 * t600 * t3533;
    let t13181 = t97 * t2083;
    let t13202 = t105 * t2091;
    let t13296 = t10281 - t10282 - t7656 + t7659 + t10283 - t10284 - t7665 + t7668 + t10285 - t10286 - t7676;
    (t13154, t13157, t13159, t13181, t13202, t13296)
}
