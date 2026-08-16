//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 889/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk889(t10316: f64, t3230: f64, t2225: f64, t3198: f64, t2217: f64, t10284: f64, t10290: f64, t10295: f64, t10299: f64, t10303: f64, t10306: f64, t10311: f64, t10314: f64) -> f64 {
    let t10317 = t3230 * t10316;
    let t10319 = t2225 * t3198;
    let t10321 = t2217 * t3198;
    let t10323 = -0.2087902056652481864e-5_f64 * t10284 + 0.58183124501243180478e-7_f64 * t10290 + 0.342503618217270647e-5_f64 * t10295 + 0.2087902056652481864e-5_f64 * t10299 + 0.11416787273909021566e-6_f64 * t10303 + 0.18788769913633132635e-4_f64 * t10306 + 0.33406432906439709826e-4_f64 * t10311 + 0.11742981196020707897e-5_f64 * t10314 + 0.82073827867876094584e-5_f64 * t10317 + 0.33406432906439709826e-4_f64 * t10319 - 0.77948343448359322927e-4_f64 * t10321;
    t10323
}
