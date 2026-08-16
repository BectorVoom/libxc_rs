//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 927/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk927(t321: f64, t9876: f64, t262: f64, t7198: f64, t1469: f64, t2318: f64, t34976: f64, t40145: f64, t2281: f64, t35039: f64, t39851: f64, t16504: f64, t552: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45418 = t9876 * t321;
    let t45419 = t262 * t45418;
    let t45420 = t7198 * t45419;
    let t45424 = t40145 * t34976 * t2318 * t1469;
    let t45428 = t39851 * t35039 * t2281 * t1469;
    let t45432 = t39851 * t16504 * t552 * t1469;
    (t45418, t45419, t45420, t45424, t45428, t45432)
}
