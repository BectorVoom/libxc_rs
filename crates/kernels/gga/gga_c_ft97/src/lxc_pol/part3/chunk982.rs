//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 982/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk982<F: Float>(t10373: F, t13643: F, t18051: F, t18055: F, t18058: F, t18061: F, t18064: F, t18066: F, t18068: F, t18071: F, t18074: F, t18077: F, t18081: F) -> F {
    let t19229 = F::cast_from(0.16669500273148148149e-1_f64) * t18051 + F::cast_from(0.59269334304526748974e-1_f64) * t13643 - F::cast_from(0.13335600218518518519e0_f64) * t18055 + F::cast_from(0.51860667516460905352e-1_f64) * t18058 - F::cast_from(0.8890400145679012346e-1_f64) * t18061 + F::cast_from(0.10001700163888888889e0_f64) * t18064 + F::cast_from(0.17780800291358024692e0_f64) * t18066 - F::cast_from(0.11853866860905349795e0_f64) * t18068 + F::cast_from(0.13335600218518518519e0_f64) * t18071 - F::cast_from(0.33339000546296296298e-1_f64) * t18074 + F::cast_from(0.22226000364197530865e-1_f64) * t18077 + t10373 + F::cast_from(0.16299066933744855968e0_f64) * t18081;
    t19229
}
