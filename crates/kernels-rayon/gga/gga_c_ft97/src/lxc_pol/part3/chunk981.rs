//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 981/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk981(t19201: f64, t291: f64, t800: f64, t13600: f64, t13629: f64, t13635: f64, t13648: f64, t14788: f64, t18032: f64, t18035: f64, t18038: f64, t18040: f64, t18044: f64, t18046: f64, t9639: f64) -> (f64, f64) {
    let t19202 = t291 * t19201;
    let t19203 = t800 * t19202;
    let t19216 = -t14788 + 0.14817333576131687244e-1_f64 * t13600 - 0.3704333394032921811e-2_f64 * t9639 - 0.22226000364197530866e-1_f64 * t13629 - 0.29634667152263374487e-1_f64 * t13635 - 0.7408666788065843622e-2_f64 * t13648 + 0.55565000910493827163e-2_f64 * t18032 + 0.74086667880658436217e-2_f64 * t18035 - 0.11113000182098765433e-1_f64 * t18038 - 0.29634667152263374487e-1_f64 * t18040 + 0.16299066933744855968e0_f64 * t18044 + 0.17780800291358024692e0_f64 * t18046;
    (t19203, t19216)
}
