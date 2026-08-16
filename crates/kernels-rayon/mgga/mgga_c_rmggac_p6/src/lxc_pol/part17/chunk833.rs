//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 833/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk833(t262: f64, t41055: f64, t2068: f64, t2123: f64, t551: f64, t305: f64, t38674: f64, t118: f64, t25809: f64, t39692: f64, t5271: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41056 = t262 * t41055;
    let t41057 = t2068 * t41056;
    let t41059 = t2123 * t551;
    let t41114 = t305 * t38674;
    let t41115 = 0.79828278012425390426e-1_f64 * t41114;
    let t41116 = t118 * t25809;
    let t41120 = t5271 * t39692;
    let t41122 = t2123 * t558;
    (t41056, t41057, t41059, t41115, t41116, t41120, t41122)
}
