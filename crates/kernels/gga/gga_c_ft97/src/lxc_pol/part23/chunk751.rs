//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 751/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk751<F: Float>(t19201: F, t291: F, t800: F, t13600: F, t13629: F, t13635: F, t13648: F, t14788: F, t18032: F, t18035: F, t18038: F, t18040: F, t18044: F, t18046: F, t9639: F, t10373: F, t13643: F, t18051: F, t18055: F, t18058: F, t18061: F, t18064: F, t18066: F, t18068: F, t18071: F, t18074: F, t18077: F, t18081: F) -> (F, F, F) {
    let t19202 = t291 * t19201;
    let t19203 = t800 * t19202;
    let t19216 = -t14788 + 0.14817333576131687244e-1 * t13600 - 0.3704333394032921811e-2 * t9639 - 0.22226000364197530866e-1 * t13629 - 0.29634667152263374487e-1 * t13635 - 0.7408666788065843622e-2 * t13648 + 0.55565000910493827163e-2 * t18032 + 0.74086667880658436217e-2 * t18035 - 0.11113000182098765433e-1 * t18038 - 0.29634667152263374487e-1 * t18040 + 0.16299066933744855968e0 * t18044 + 0.17780800291358024692e0 * t18046;
    let t19229 = 0.16669500273148148149e-1 * t18051 + 0.59269334304526748974e-1 * t13643 - 0.13335600218518518519e0 * t18055 + 0.51860667516460905352e-1 * t18058 - 0.8890400145679012346e-1 * t18061 + 0.10001700163888888889e0 * t18064 + 0.17780800291358024692e0 * t18066 - 0.11853866860905349795e0 * t18068 + 0.13335600218518518519e0 * t18071 - 0.33339000546296296298e-1 * t18074 + 0.22226000364197530865e-1 * t18077 + t10373 + 0.16299066933744855968e0 * t18081;
    (t19203, t19216, t19229)
}
