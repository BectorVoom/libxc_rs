//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1134/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1134(t12939: f64, t1625: f64, t209: f64, t736: f64, t4188: f64, t5895: f64, t12344: f64, t2016: f64, t2118: f64, t1943: f64, t38630: f64, t17329: f64, sigma2: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40662 = t1625 * t12939;
    let t44682 = t209 * t736;
    let t48044 = t5895 * t4188;
    let t48058 = t2016 * t12344;
    let t51097 = t2118 * t12939;
    let t51121 = t1943 * t38630;
    let t51125 = t17329 * sigma2;
    (t40662, t44682, t48044, t48058, t51097, t51121, t51125)
}
