//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 981/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk981<F: Float>(t19201: F, t291: F, t800: F, t13600: F, t13629: F, t13635: F, t13648: F, t14788: F, t18032: F, t18035: F, t18038: F, t18040: F, t18044: F, t18046: F, t9639: F) -> (F, F) {
    let t19202 = t291 * t19201;
    let t19203 = t800 * t19202;
    let t19216 = -t14788 + F::cast_from(0.14817333576131687244e-1_f64) * t13600 - F::cast_from(0.3704333394032921811e-2_f64) * t9639 - F::cast_from(0.22226000364197530866e-1_f64) * t13629 - F::cast_from(0.29634667152263374487e-1_f64) * t13635 - F::cast_from(0.7408666788065843622e-2_f64) * t13648 + F::cast_from(0.55565000910493827163e-2_f64) * t18032 + F::cast_from(0.74086667880658436217e-2_f64) * t18035 - F::cast_from(0.11113000182098765433e-1_f64) * t18038 - F::cast_from(0.29634667152263374487e-1_f64) * t18040 + F::cast_from(0.16299066933744855968e0_f64) * t18044 + F::cast_from(0.17780800291358024692e0_f64) * t18046;
    (t19203, t19216)
}
