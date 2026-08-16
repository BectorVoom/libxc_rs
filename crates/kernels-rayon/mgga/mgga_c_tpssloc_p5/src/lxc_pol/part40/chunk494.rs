//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 40 (v4rho3tau_4) CSE chunk 494/1303 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part40_v4rho3tau_4_chunk494(t265: f64, t504: f64, t1238: f64, t1721: f64, t1752: f64, t1761: f64, t498: f64, t1256: f64, t1534: f64, t1659: f64, t1673: f64, t1699: f64, t1701: f64, t1705: f64, t193: f64, t336: f64) -> (f64, f64) {
    let t505 = t265 < t504;
    let t1763 = -t1238 * t1761 + t1721 * t498 + t1752 * t498;
    let t1768 = piecewise3(t505, t1256 * t1763 * t193 * t336 - t1659 + t1673 + t1699 + t1701 - t1705, t1534);
    (t1763, t1768)
}
