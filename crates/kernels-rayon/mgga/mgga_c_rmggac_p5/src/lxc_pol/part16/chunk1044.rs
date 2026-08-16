//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 1044/1158 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk1044(t34847: f64, t9990: f64, t1528: f64, t236: f64, t615: f64, t7230: f64, t7231: f64, t4044: f64, t46055: f64, t5058: f64, t8639: f64, t8642: f64) -> (f64, f64, f64, f64) {
    let t47767 = t34847 * t9990;
    let t47772 = t7230 * t7231 * t236 * t1528 * t615;
    let t47774 = t4044 * t46055;
    let t47785 = t5058 * t8639 * t8642;
    (t47767, t47772, t47774, t47785)
}
