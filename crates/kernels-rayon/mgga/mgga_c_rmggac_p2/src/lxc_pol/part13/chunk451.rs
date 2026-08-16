//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 451/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk451(t3869: f64, t537: f64, t810: f64, t809: f64, t87: f64, t312: f64, t815: f64, t1569: f64, t816: f64, t814: f64, t90: f64, t154: f64, t1573: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4858 = t3869 * t537 * t810;
    let t4861 = t87 * t809;
    let t4862 = t815 * t312;
    let t4865 = t1569 * t816;
    let t4868 = t90 * t814;
    let t4871 = t1573 * t154;
    (t4858, t4861, t4862, t4865, t4868, t4871)
}
