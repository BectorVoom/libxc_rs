//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 837/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk837(t270: f64, t574: f64, t290: f64, t2010: f64, t7755: f64, t1664: f64, t7556: f64, t2012: f64, t7349: f64, t2019: f64, t640: f64, t7764: f64) -> (f64, f64, f64) {
    let t38815 = t574 * t270;
    let t38816 = t290 * t38815;
    let t38818 = t2010 * t7755 * t38816;
    let t38820 = t1664 * t7556;
    let t38822 = t7349 * t2012 * t38820;
    let t38826 = t2019 * t7764 * t640 * t38815;
    (t38818, t38822, t38826)
}
