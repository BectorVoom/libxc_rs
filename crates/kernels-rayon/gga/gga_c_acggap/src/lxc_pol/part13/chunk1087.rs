//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1087/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1087(t1016: f64, t2060: f64, t361: f64, t8928: f64, t2030: f64, t20559: f64, t7502: f64, t15695: f64, t7450: f64, t8915: f64, t17752: f64, t8919: f64) -> (f64, f64, f64, f64) {
    let t34920 = t2060 * t361 * t1016 * t8928;
    let t34923 = t2030 * t20559 * t7502;
    let t34926 = t7450 * t15695 * t8915;
    let t34929 = t2030 * t17752 * t8919;
    (t34920, t34923, t34926, t34929)
}
