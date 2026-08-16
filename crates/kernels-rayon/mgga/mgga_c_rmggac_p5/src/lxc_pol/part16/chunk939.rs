//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 939/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk939(t1835: f64, t1979: f64, t1982: f64, t201: f64, t457: f64, t2191: f64, t9932: f64, t9935: f64, t1986: f64, t6592: f64, t675: f64, t1743: f64, t352: f64) -> (f64, f64, f64, f64, f64) {
    let t45608 = t1835 * t457 * t201 * t1979 * t1982;
    let t45610 = t2191 * t9932;
    let t45614 = t2191 * t9935;
    let t45617 = t675 * t1986 * t6592;
    let t45622 = t1743 * t352;
    (t45608, t45610, t45614, t45617, t45622)
}
