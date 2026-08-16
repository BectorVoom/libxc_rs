//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 461/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk461(t504: f64, t837: f64, t4035: f64, t529: f64, t1368: f64, t866: f64, t551: f64, t874: f64, t876: f64, t559: f64, t833: f64, t124: f64, t235: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t5019 = t504 * t837;
    let t5026 = t4035 * t529;
    let t5029 = t1368 * t866;
    let t5032 = t874 * t551;
    let t5033 = t5032 * t876;
    let t5041 = t559 * t833;
    let t5048 = t235 * t124;
    (t5019, t5026, t5029, t5033, t5041, t5048)
}
