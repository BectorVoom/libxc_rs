//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 784/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk784(t1444: f64, t160: f64, t833: f64, t2645: f64, t4061: f64, t1445: f64, t2642: f64, t1441: f64, t532: f64, t450: f64, t4075: f64, t743: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11951 = t160 * t1444;
    let t11952 = t11951 * t833;
    let t11954 = t4061 * t2645;
    let t11958 = t1445 * t2642;
    let t11960 = t1441 * t833;
    let t11962 = t532 * t2645;
    let t11966 = t160 * t450;
    let t11967 = 0.71734315950379065738e-1_f64 * t11966;
    let t11974 = t743 * t4075;
    (t11951, t11952, t11954, t11958, t11960, t11962, t11966, t11967, t11974)
}
