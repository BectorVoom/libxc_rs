//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1080/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1080(t133: f64, t19241: f64, t19265: f64, t120: f64, t4573: f64, t2911: f64, t5870: f64, t8236: f64, t1504: f64, t1533: f64, t19270: f64, t19274: f64, t19307: f64, t19338: f64, t19349: f64, t19351: f64, t19362: f64, t19365: f64, t19373: f64, t19385: f64, t2912: f64, t481: f64, t5645: f64, t8231: f64) -> f64 {
    let t19429 = t133 * t19241;
    let t19431 = t133 * t19265;
    let t19439 = 0.29801938271604938271e1_f64 * t133 * t4573 * t120;
    let t19449 = t2911 * t8236 * t5870;
    let t19451 = -0.1724255e1_f64 * t133 * t19307 + 0.1379404e2_f64 * t19429 + 0.2758808e2_f64 * t19431 + 0.1034553e3_f64 * t133 * t19270 + 0.15518295e2_f64 * t133 * t19274 - t19338 + t19439 - 0.12414636e3_f64 * t2911 * t8231 * t1504 * t1533 + 0.2069106e2_f64 * t2911 * t2912 * t481 * t5645 - 0.2069106e2_f64 * t19449 - t19349 + t19351 + t19362 + t19365 - t19373 + t19385;
    t19451
}
