//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 435/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk435(t1193: f64, t4504: f64, t463: f64, t205: f64, t1184: f64, t209: f64, t1194: f64, t1465: f64, t4461: f64, t465: f64, t479: f64, t198: f64, t2184: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t4505 = t1193 * t4504;
    let t4516 = t463 * t463;
    let t4517 = 1.0_f64 / t4516;
    let t4518 = t205 * t4517;
    let t4522 = t1184 * t209;
    let t4544 = t1465 * t1194;
    let t4555 = t465 * t4461;
    let t4556 = t4555 * t479;
    let t4558 = t2184 * t198;
    (t4505, t4517, t4518, t4522, t4544, t4555, t4556, t4558)
}
