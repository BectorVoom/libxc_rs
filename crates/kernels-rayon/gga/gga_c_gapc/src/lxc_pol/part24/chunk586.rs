//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 586/1327 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk586(t3129: f64, t3134: f64, t3145: f64, t3147: f64, t3150: f64, t3158: f64, t3161: f64, t3164: f64, t3166: f64, t3168: f64, t3173: f64, t3175: f64) -> f64 {
    let t3535 = -0.75883739738679928909e-7_f64 * t3129 + 0.1349212892553729136e-6_f64 * t3134 - 0.49240895655712845849e-7_f64 * t3145 + 0.27801896084645508334e-2_f64 * t3147 + 0.20241536458333333335e-4_f64 * t3150 + 0.29518907335069444447e-5_f64 * t3158 + 0.27801896084645508334e-2_f64 * t3161 - 0.28985453471303521736e-5_f64 * t3164 - 0.10120768229166666668e-3_f64 * t3166 + 0.12380568050579229813e-5_f64 * t3168 - 0.69504740211613770835e-4_f64 * t3173 - 0.64871090864172852779e-2_f64 * t3175;
    t3535
}
