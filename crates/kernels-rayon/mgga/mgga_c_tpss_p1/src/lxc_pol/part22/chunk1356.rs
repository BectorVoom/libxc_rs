//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1356/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1356(t1006: f64, t1692: f64, t1812: f64, t18247: f64, t18254: f64, t18271: f64, t20012: f64, t20018: f64, t20510: f64, t20514: f64, t2439: f64, t33: f64, t5671: f64, t5678: f64, t5853: f64, t62610: f64, t6354: f64, t64896: f64, t64982: f64, t65030: f64, t66235: f64, t66262: f64, t66281: f64, t66311: f64, t66317: f64, t66604: f64) -> f64 {
    let t66796 = -t1692 * t20514 * t18271 / 2.0_f64 - t1692 * t5853 * t65030 / 2.0_f64 - t66235 + t1692 * t20510 * t1006 - 3.0_f64 * t62610 * t20018 + 6.0_f64 * t66311 * t20012 - t66262 - 3.0_f64 * t66317 * t18247 - t1692 * t66281 * t5678 + 3.0_f64 * t2439 * t1812 * t64896 + 3.0_f64 / 2.0_f64 * t2439 * t6354 * t18254 - t1692 * t5853 * t64982 / 2.0_f64 + 3.0_f64 * t2439 * t20510 * t5671 + t1692 * t66604 * t33 / 2.0_f64;
    t66796
}
