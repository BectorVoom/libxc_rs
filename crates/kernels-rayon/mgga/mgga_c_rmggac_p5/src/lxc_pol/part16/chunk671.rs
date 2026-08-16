//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 671/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk671(t2010: f64, t9723: f64, t1665: f64, t2415: f64, t1948: f64, t1986: f64, t675: f64, t589: f64, t597: f64, t201: f64, t1979: f64, t1982: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9724 = t2010 * t9723;
    let t9726 = t2415 * t1665;
    let t9727 = t2010 * t9726;
    let t9731 = t1986 * t1948;
    let t9732 = t675 * t9731;
    let t9734 = t589 * t597;
    let t9735 = t9734 * t201;
    let t9737 = t9735 * t1979 * t1982;
    (t9724, t9726, t9727, t9731, t9732, t9734, t9735, t9737)
}
