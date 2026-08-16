//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 385/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk385(t1307: f64, t1451: f64, t104: f64, t111: f64, t120: f64, t1404: f64, t1424: f64, t1429: f64, t1431: f64, t1436: f64, t1438: f64, t1442: f64, t1445: f64, t1450: f64, t833: f64) -> (f64, f64) {
    let t1452 = t1451 * t1307;
    let t1455 = t1424 + 0.11955719325063177623e-1_f64 * t1404 * t833 - t1429 - 0.3513e-2_f64 * t104 * t1431 + t1436 + 0.7925e-3_f64 * t111 * t1438 - t1442 - 0.5179538907796306876e-4_f64 * t1445 * t833 + t1450 + 0.50413125e-5_f64 * t120 * t1452;
    (t1452, t1455)
}
