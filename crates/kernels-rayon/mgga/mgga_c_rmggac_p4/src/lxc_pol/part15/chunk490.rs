//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 490/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk490(t221: f64, t446: f64, t6182: f64, t1190: f64, t1891: f64, t1895: f64, t1870: f64, t4569: f64, t1835: f64, t6: f64, t1515: f64, t1839: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6184 = t221 * t6182 * t446;
    let t6188 = t1190 * t1891;
    let t6190 = t1190 * t1895;
    let t6192 = t4569 * t1870;
    let t6194 = t6 * t1835;
    let t6196 = t1515 * t6194 * t446;
    let t6199 = t6 * t1839;
    (t6184, t6188, t6190, t6192, t6196, t6199)
}
