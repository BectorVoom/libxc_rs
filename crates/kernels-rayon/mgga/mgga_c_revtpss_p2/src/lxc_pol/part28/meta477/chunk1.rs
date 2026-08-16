//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1811/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1811(t1976: f64, t3325: f64, t7160: f64, t3075: f64, t7145: f64, t1982: f64, t3259: f64, t1972: f64, t3223: f64) -> (f64, f64, f64, f64) {
    let t25479 = t1976 * t3325;
    let t25480 = t7160 * t25479;
    let t25483 = t1976 * t3075;
    let t25484 = t7145 * t25483;
    let t25487 = t1982 * t3259;
    let t25490 = t3223 * t1972;
    (t25480, t25484, t25487, t25490)
}
