//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 644/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk644(t1616: f64, t1986: f64, t675: f64, t2191: f64, t2310: f64, t1654: f64, t446: f64, t597: f64, t201: f64, t1979: f64, t1982: f64, t1451: f64, t194: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8592 = t1986 * t1616;
    let t8593 = t675 * t8592;
    let t8595 = t2191 * t2310;
    let t8597 = t1986 * t1654;
    let t8598 = t675 * t8597;
    let t8601 = t446 * t597;
    let t8602 = t8601 * t201;
    let t8604 = t8602 * t1979 * t1982;
    let t8607 = t194 * t1451;
    (t8592, t8593, t8595, t8597, t8598, t8601, t8602, t8604, t8607)
}
