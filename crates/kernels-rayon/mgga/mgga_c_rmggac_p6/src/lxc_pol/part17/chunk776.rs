//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 776/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk776(t1969: f64, t34846: f64, t7345: f64, t7927: f64, t35207: f64, t7354: f64, t2019: f64, t2165: f64, t7926: f64, t2169: f64, t7334: f64, t7932: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t36772 = t34846 * t1969;
    let t36796 = t7345 * t7927;
    let t36797 = 0.12195059916630011326e-2_f64 * t36796;
    let t36801 = t35207 * t7354;
    let t36802 = 0.5854811038705731867e-3_f64 * t36801;
    let t36804 = t2019 * t7926 * t2165;
    let t36809 = t2019 * t7926 * t2169;
    let t36912 = t7334 * t7932;
    (t36772, t36797, t36802, t36804, t36809, t36912)
}
