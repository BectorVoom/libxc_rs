//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 990/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk990(t14578: f64, t1504: f64, t4214: f64, t469: f64, t4205: f64, t1513: f64, t4301: f64, t1501: f64, t4182: f64, t1488: f64, t4312: f64, t1487: f64) -> (f64, f64, f64, f64, f64) {
    let t14579 = t1504 * t14578;
    let t14581 = t4214 * t469;
    let t14582 = t14581 * t4205;
    let t14584 = t4301 * t1513;
    let t14586 = t1501 * t4182;
    let t14588 = t4312 * t1488;
    let t14589 = t1487 * t14588;
    (t14579, t14582, t14584, t14586, t14589)
}
