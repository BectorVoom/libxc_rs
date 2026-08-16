//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 417/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk417(t184: f64, t4068: f64, t1328: f64, t74: f64, t433: f64, t959: f64, t945: f64, t249: f64, t980: f64, t1042: f64, t388: f64, t5: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t4069 = t4068 * t184;
    let t4071 = 1.0_f64 / t1328;
    let t4072 = t74 * t4071;
    let t4074 = 120.0_f64 * t4072 * t184;
    let t4075 = t959 * t433;
    let t4077 = 0.21687162600603479684e-1_f64 * t945 * t4075;
    let t4078 = t249 * t980;
    let t4080 = 0.32530743900905219526e-1_f64 * t945 * t4078;
    let t4081 = t249 * t1042;
    let t4083 = 0.48159733137676571078e0_f64 * t945 * t4081;
    let t4084 = t388 * t5;
    (t4069, t4074, t4077, t4080, t4083, t4084)
}
