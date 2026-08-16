//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 982/1032 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk982(t10373: f64, t13643: f64, t18051: f64, t18055: f64, t18058: f64, t18061: f64, t18064: f64, t18066: f64, t18068: f64, t18071: f64, t18074: f64, t18077: f64, t18081: f64) -> f64 {
    let t19229 = 0.16669500273148148149e-1_f64 * t18051 + 0.59269334304526748974e-1_f64 * t13643 - 0.13335600218518518519e0_f64 * t18055 + 0.51860667516460905352e-1_f64 * t18058 - 0.8890400145679012346e-1_f64 * t18061 + 0.10001700163888888889e0_f64 * t18064 + 0.17780800291358024692e0_f64 * t18066 - 0.11853866860905349795e0_f64 * t18068 + 0.13335600218518518519e0_f64 * t18071 - 0.33339000546296296298e-1_f64 * t18074 + 0.22226000364197530865e-1_f64 * t18077 + t10373 + 0.16299066933744855968e0_f64 * t18081;
    t19229
}
