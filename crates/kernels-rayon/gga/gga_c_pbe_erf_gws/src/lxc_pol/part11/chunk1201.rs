//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1201/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1201(t145: f64, t169: f64, t171: f64, t18995: f64, t18998: f64, t19001: f64, t19004: f64, t19035: f64, t19044: f64, t242: f64, t26031: f64, t26034: f64, t26038: f64, t26051: f64, t26061: f64, t34237: f64, t34244: f64, t34254: f64, t34274: f64, t42876: f64, t42880: f64, t42891: f64, t48321: f64, t48520: f64) -> f64 {
    let t48908 = 0.533250677421793803e-1_f64 * t145 * t48520 + 0.63671331549358746541e0_f64 * t26061 - 0.16979021746495665744e1_f64 * t26031 + t18995 + 0.63671331549358746541e0_f64 * t34244 - 0.19101399464807623963e0_f64 * t34254 - 0.4266005419374350424e0_f64 * t42876 - t18998 - 0.51192065032492205088e1_f64 * t26051 + t19001 - t19004 - 0.12734266309871749308e0_f64 * t26034 - 0.12734266309871749308e0_f64 * t42891 - 0.84895108732478328721e0_f64 * t34237 + 0.20752137690161369243e1_f64 * t26038 - 0.31835665774679373271e-1_f64 * t169 * t171 * t48321 * t242 + t19035 - t19044 + 0.2122377718311958218e0_f64 * t42880 + 0.19197024387184576908e1_f64 * t34274;
    t48908
}
