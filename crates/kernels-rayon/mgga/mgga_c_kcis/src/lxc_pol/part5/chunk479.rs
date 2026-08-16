//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 479/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk479(t1430: f64, t1889: f64, t1437: f64, t1451: f64, t104: f64, t111: f64, t120: f64, t1404: f64, t1424: f64, t1429: f64, t1436: f64, t1442: f64, t1445: f64, t1450: f64, t1650: f64) -> (f64, f64, f64, f64) {
    let t1968 = t1430 * t1889;
    let t1971 = t1437 * t1889;
    let t1976 = t1451 * t1889;
    let t1979 = t1424 + 0.11955719325063177623e-1_f64 * t1404 * t1650 - t1429 - 0.3513e-2_f64 * t104 * t1968 + t1436 + 0.7925e-3_f64 * t111 * t1971 - t1442 - 0.5179538907796306876e-4_f64 * t1445 * t1650 + t1450 + 0.50413125e-5_f64 * t120 * t1976;
    (t1968, t1971, t1976, t1979)
}
