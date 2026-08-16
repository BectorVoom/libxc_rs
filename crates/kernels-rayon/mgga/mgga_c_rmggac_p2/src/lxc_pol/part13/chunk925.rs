//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 925/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk925(t1987: f64, t9087: f64, t2004: f64, t9090: f64, t2007: f64, t1990: f64, t1173: f64, t674: f64, t9085: f64, t1997: f64, t7696: f64, t8676: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40347 = t9087 * t1987;
    let t40349 = t9090 * t2004;
    let t40351 = t9090 * t2007;
    let t40354 = t9090 * t1987;
    let t40356 = t9090 * t1990;
    let t40359 = t9085 * t1173 * t674;
    let t40360 = t40359 * t1997;
    let t40362 = t8676 * t7696;
    (t40347, t40349, t40351, t40354, t40356, t40360, t40362)
}
