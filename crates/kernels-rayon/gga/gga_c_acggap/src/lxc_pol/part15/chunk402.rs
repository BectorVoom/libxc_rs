//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 402/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk402(t1247: f64, t1815: f64, t1651: f64, t525: f64, t1839: f64, t456: f64, t1844: f64, t182: f64, t1907: f64, t119: f64, t1226: f64, t1228: f64, t1235: f64, t1246: f64, t151: f64, t1627: f64, t1631: f64, t1645: f64, t1649: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1922 = t1247 * t1815;
    let t1925 = t1651 * t525;
    let t1928 = t456 * t1839;
    let t1931 = t456 * t1844;
    let t1934 = t182 * t1907;
    let t1937 = t1226 - t1228 - 0.13170898365871023197e1_f64 * t1627 + 0.13170898365871023197e1_f64 * t1645 + t1235 + 0.13170898365871023197e1_f64 * t1631 - 0.13170898365871023197e1_f64 * t1649 - t1246 + 0.13170898365871023197e1_f64 * t151 * t1922 - 0.13170898365871023197e1_f64 * t151 * t1925 - 0.65854491829355115987e0_f64 * t151 * t1928 - 0.65854491829355115987e0_f64 * t151 * t1931 + 0.65854491829355115987e0_f64 * t119 * t1934;
    (t1922, t1925, t1928, t1931, t1934, t1937)
}
