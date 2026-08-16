//! MGGA_C_REVTPSS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1362/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part5_v3rho3_2_chunk1362(t20248: f64, t21657: f64, t118: f64, t1310: f64, t13426: f64, t1502: f64, t1519: f64, t18220: f64, t18227: f64, t18232: f64, t18235: f64, t18242: f64, t18245: f64, t1843: f64, t2322: f64, t4246: f64, t4248: f64, t4254: f64, t4257: f64, t508: f64, t5517: f64, t5877: f64, t5884: f64, t5921: f64, t651: f64, t671: f64) -> f64 {
    let t21658 = t20248 + t21657;
    let t21660 = -t118 * t21658 - t1310 * t5877 - 2.0_f64 * t1310 * t5884 - 4.0_f64 * t13426 * t1519 - 2.0_f64 * t1502 * t5517 - 4.0_f64 * t1519 * t18227 - 2.0_f64 * t18220 * t508 - 2.0_f64 * t18232 * t651 - 4.0_f64 * t18235 * t651 - 2.0_f64 * t18242 * t651 - 2.0_f64 * t18245 * t671 - 2.0_f64 * t1843 * t4246 - 2.0_f64 * t2322 * t5921 - 4.0_f64 * t4248 * t4257 - 2.0_f64 * t4254 * t5921;
    t21660
}
