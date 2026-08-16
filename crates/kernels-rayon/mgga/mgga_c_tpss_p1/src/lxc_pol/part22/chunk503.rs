//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 503/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk503(t1993: f64, t70: f64, t582: f64, t602: f64, t350: f64, t41: f64, t47: f64, t1985: f64, t1992: f64, t48: f64, t59: f64, t60: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1994 = t1993 * t70;
    let t1997 = t582 * t602;
    let t2003 = 1.0_f64 / t41 / t350;
    let t2004 = sigma0 * t2003;
    let t2009 = 1.0_f64 / t47;
    let t2010 = t2009 * t1985;
    let t2013 = t48 * t1992;
    let t2016 = 1.0_f64 / t59;
    let t2017 = t2016 * t1985;
    let t2020 = t60 * t1992;
    (t1994, t1997, t2004, t2009, t2010, t2013, t2016, t2017, t2020)
}
