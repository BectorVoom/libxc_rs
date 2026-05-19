//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1053/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1053<F: Float>(t145: F, t16580: F, t169: F, t171: F, t18987: F, t18995: F, t18998: F, t19001: F, t19004: F, t19007: F, t19010: F, t19013: F, t19020: F, t19023: F, t19026: F, t19028: F, t19031: F, t19035: F, t19037: F, t19040: F, t19044: F, t19045: F, t19047: F, t242: F) -> F {
    let t19051 = t18995 - t18998 + t19001 - t19004 + F::cast_from(0.2122377718311958218e0_f64) * t19007 + F::cast_from(0.63671331549358746541e0_f64) * t19010 + F::cast_from(0.63671331549358746541e0_f64) * t19013 - F::cast_from(0.31835665774679373271e-1_f64) * t169 * t171 * t16580 * t242 - F::cast_from(0.12734266309871749308e0_f64) * t19020 - F::cast_from(0.19101399464807623963e0_f64) * t19023 - F::cast_from(0.12734266309871749308e0_f64) * t19026 - F::cast_from(0.51192065032492205088e1_f64) * t19028 + F::cast_from(0.20752137690161369243e1_f64) * t19031 + t19035 - F::cast_from(0.84895108732478328721e0_f64) * t19037 - F::cast_from(0.16979021746495665744e1_f64) * t19040 - t19044 + F::cast_from(0.19197024387184576908e1_f64) * t19045 - F::cast_from(0.4266005419374350424e0_f64) * t19047 + F::cast_from(0.533250677421793803e-1_f64) * t145 * t18987;
    t19051
}
