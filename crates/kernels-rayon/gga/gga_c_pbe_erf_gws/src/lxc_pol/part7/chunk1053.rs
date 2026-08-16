//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1053/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1053(t145: f64, t16580: f64, t169: f64, t171: f64, t18987: f64, t18995: f64, t18998: f64, t19001: f64, t19004: f64, t19007: f64, t19010: f64, t19013: f64, t19020: f64, t19023: f64, t19026: f64, t19028: f64, t19031: f64, t19035: f64, t19037: f64, t19040: f64, t19044: f64, t19045: f64, t19047: f64, t242: f64) -> f64 {
    let t19051 = t18995 - t18998 + t19001 - t19004 + 0.2122377718311958218e0_f64 * t19007 + 0.63671331549358746541e0_f64 * t19010 + 0.63671331549358746541e0_f64 * t19013 - 0.31835665774679373271e-1_f64 * t169 * t171 * t16580 * t242 - 0.12734266309871749308e0_f64 * t19020 - 0.19101399464807623963e0_f64 * t19023 - 0.12734266309871749308e0_f64 * t19026 - 0.51192065032492205088e1_f64 * t19028 + 0.20752137690161369243e1_f64 * t19031 + t19035 - 0.84895108732478328721e0_f64 * t19037 - 0.16979021746495665744e1_f64 * t19040 - t19044 + 0.19197024387184576908e1_f64 * t19045 - 0.4266005419374350424e0_f64 * t19047 + 0.533250677421793803e-1_f64 * t145 * t18987;
    t19051
}
