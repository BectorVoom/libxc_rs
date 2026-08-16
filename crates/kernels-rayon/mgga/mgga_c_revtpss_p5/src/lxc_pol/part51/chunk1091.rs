//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 1091/1200 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk1091(t125211: f64, t125213: f64, t125215: f64, t125217: f64, t125223: f64, t125343: f64, t125361: f64, t125392: f64, t1310: f64, t1453: f64, t1843: f64, t1911: f64, t2007: f64, t28160: f64, t32161: f64, t32179: f64, t33578: f64, t33580: f64, t33583: f64, t33630: f64, t33647: f64, t33903: f64, t508: f64, t5517: f64, t569: f64, t649: f64, t7221: f64, t7725: f64, t8447: f64) -> f64 {
    let t125401 = -t125211 - t125213 - t125215 - t125217 - t33578 - t33580 - t33583 - 2.0_f64 * t28160 * t2007 - 2.0_f64 * t7725 * t7221 - t649 * t33903 + 2.0_f64 * t125223 + (t125361 + t125392) * t569 + t32179 * t1911 + t33647 * t1453 - t32161 * t1843 - t8447 * t5517 - t125343 * t508 - t33630 * t1310;
    t125401
}
