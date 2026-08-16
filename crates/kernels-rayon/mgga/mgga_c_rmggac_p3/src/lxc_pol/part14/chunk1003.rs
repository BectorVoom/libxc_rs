//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 1003/1089 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk1003(t38745: f64, t5271: f64, t39670: f64, t5162: f64, t39674: f64, t4669: f64, t305: f64, t38674: f64, t118: f64, t25809: f64, t39692: f64, t2123: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t41101 = t5271 * t38745;
    let t41106 = t5162 * t39670;
    let t41108 = t4669 * t39674;
    let t41114 = t305 * t38674;
    let t41115 = 0.79828278012425390426e-1_f64 * t41114;
    let t41116 = t118 * t25809;
    let t41120 = t5271 * t39692;
    let t41122 = t2123 * t558;
    (t41101, t41106, t41108, t41115, t41116, t41120, t41122)
}
