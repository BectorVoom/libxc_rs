//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 814/1088 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk814(t2007: f64, t38351: f64, t38355: f64, t2310: f64, t36542: f64, t7720: f64, t8597: f64, t674: f64, t7715: f64, t8601: f64, t1997: f64, t8607: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38361 = t38351 * t2007;
    let t38363 = t38355 * t2007;
    let t38365 = t36542 * t2310;
    let t38367 = t7720 * t8597;
    let t38370 = t8601 * t7715 * t674;
    let t38371 = t38370 * t1997;
    let t38374 = t8607 * t7715 * t674;
    (t38361, t38363, t38365, t38367, t38371, t38374)
}
