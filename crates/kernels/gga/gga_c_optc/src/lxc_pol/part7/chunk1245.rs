//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1245/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1245<F: Float>(t2367: F, t7398: F, t930: F, t2812: F, t7983: F, t8143: F, t8233: F, t925: F, t11399: F, t123: F, t146: F, t23503: F, t24521: F, t25003: F, t25610: F, t2704: F, t2778: F, t2797: F, t2803: F, t297: F, t318: F, t323: F, t324: F, t3884: F, t3886: F, t7427: F, t7859: F, t8004: F, t8078: F, t8109: F, t8140: F, t8161: F, t8165: F, t8177: F, t953: F) -> F {
    let t25699 = t930 * t2367 * t7398;
    let t25715 = t2812 * t8143 * t7983;
    let t25721 = t8233 * t925;
    let t25729 = -F::new(0.17581974682482873924e4) * t3884 * t25610 * t3886 - F::new(0.5373941808892181044e0) * t2704 * t8161 + F::new(0.23181763972770020946e0) * t25699 - F::new(0.18545411178216016757e1) * t2797 * t8078 - F::new(0.13186481011862155443e4) * t2778 * t323 * t24521 * t123 * t297 - F::new(0.47768371634597164836e0) * t2704 * t7859 - F::new(0.2686970904446090522e0) * t953 * t25003 + F::new(0.1699996024669801536e1) * t8177 * t2803 + F::new(0.3118959061058811624e2) * t25715 - F::new(0.99111859977581099115e3) * t11399 * t8004 - F::new(0.4158612081411748832e3) * t8140 * t8165 - F::new(0.52888765211949381121e1) * t25721 + F::new(0.22477725215078486977e2) * t146 * t318 * t23503 * t324 + F::new(0.49555929988790549556e3) * t7427 * t8109;
    t25729
}
