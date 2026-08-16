//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 838/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk838(t41340: f64, t3814: f64, t40920: f64, t2115: f64, t41056: f64, t2103: f64, t41032: f64, t2100: f64, t41028: f64, t6444: f64, t8708: f64, t41055: f64, t793: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t41341 = 0.10620923284048465071e-1_f64 * t41340;
    let t41342 = t3814 * t40920;
    let t41347 = t2115 * t41056;
    let t41348 = 0.4838420607177634088e-3_f64 * t41347;
    let t41355 = t2103 * t41032;
    let t41363 = t2100 * t41028;
    let t41365 = t2115 * t41028;
    let t41371 = t6444 * t8708;
    let t41373 = t793 * t41055;
    (t41341, t41342, t41348, t41355, t41363, t41365, t41371, t41373)
}
