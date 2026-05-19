//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1080/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1080<F: Float>(t133: F, t19241: F, t19265: F, t120: F, t4573: F, t2911: F, t5870: F, t8236: F, t1504: F, t1533: F, t19270: F, t19274: F, t19307: F, t19338: F, t19349: F, t19351: F, t19362: F, t19365: F, t19373: F, t19385: F, t2912: F, t481: F, t5645: F, t8231: F) -> F {
    let t19429 = t133 * t19241;
    let t19431 = t133 * t19265;
    let t19439 = F::cast_from(0.29801938271604938271e1_f64) * t133 * t4573 * t120;
    let t19449 = t2911 * t8236 * t5870;
    let t19451 = -F::new(0.1724255e1) * t133 * t19307 + F::new(0.1379404e2) * t19429 + F::new(0.2758808e2) * t19431 + F::new(0.1034553e3) * t133 * t19270 + F::new(0.15518295e2) * t133 * t19274 - t19338 + t19439 - F::new(0.12414636e3) * t2911 * t8231 * t1504 * t1533 + F::new(0.2069106e2) * t2911 * t2912 * t481 * t5645 - F::new(0.2069106e2) * t19449 - t19349 + t19351 + t19362 + t19365 - t19373 + t19385;
    t19451
}
