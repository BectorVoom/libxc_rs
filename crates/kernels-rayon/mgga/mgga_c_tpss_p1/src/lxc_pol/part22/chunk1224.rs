//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1224/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1224(t10514: f64, t1692: f64, t1812: f64, t18802: f64, t18807: f64, t18812: f64, t198: f64, t207: f64, t2116: f64, t2133: f64, t2428: f64, t2433: f64, t2439: f64, t3552: f64, t5849: f64, t5853: f64, t750: f64, t821: f64, t823: f64) -> f64 {
    let t18847 = t18802 * t198 * t207 * t823 - 6.0_f64 * t10514 * t2439 * t5853 - 2.0_f64 * t1692 * t18807 * t821 + 2.0_f64 * t1692 * t18812 * t2433 - t1692 * t2428 * t5853 + 6.0_f64 * t1812 * t2116 * t3552 + 3.0_f64 * t1812 * t2133 * t2439 + 6.0_f64 * t2439 * t5849 * t750;
    t18847
}
