//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 854/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk854(t7166: f64, t984: f64, t28: f64, t110: f64, t1871: f64, t34415: f64, t7211: f64, t979: f64, t452: f64, t488: f64, t7274: f64, t942: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34619 = t7166 * t984;
    let t34620 = t28 * t34619;
    let t34624 = t1871 * t110 * t34415;
    let t34627 = t7211 * t979;
    let t34629 = t452 * t488 * t34627;
    let t34632 = t7274 * t942;
    (t34619, t34620, t34624, t34627, t34629, t34632)
}
