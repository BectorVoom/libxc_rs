//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1245/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1245(t2367: f64, t7398: f64, t930: f64, t2812: f64, t7983: f64, t8143: f64, t8233: f64, t925: f64, t11399: f64, t123: f64, t146: f64, t23503: f64, t24521: f64, t25003: f64, t25610: f64, t2704: f64, t2778: f64, t2797: f64, t2803: f64, t297: f64, t318: f64, t323: f64, t324: f64, t3884: f64, t3886: f64, t7427: f64, t7859: f64, t8004: f64, t8078: f64, t8109: f64, t8140: f64, t8161: f64, t8165: f64, t8177: f64, t953: f64) -> f64 {
    let t25699 = t930 * t2367 * t7398;
    let t25715 = t2812 * t8143 * t7983;
    let t25721 = t8233 * t925;
    let t25729 = -0.17581974682482873924e4_f64 * t3884 * t25610 * t3886 - 0.5373941808892181044e0_f64 * t2704 * t8161 + 0.23181763972770020946e0_f64 * t25699 - 0.18545411178216016757e1_f64 * t2797 * t8078 - 0.13186481011862155443e4_f64 * t2778 * t323 * t24521 * t123 * t297 - 0.47768371634597164836e0_f64 * t2704 * t7859 - 0.2686970904446090522e0_f64 * t953 * t25003 + 0.1699996024669801536e1_f64 * t8177 * t2803 + 0.3118959061058811624e2_f64 * t25715 - 0.99111859977581099115e3_f64 * t11399 * t8004 - 0.4158612081411748832e3_f64 * t8140 * t8165 - 0.52888765211949381121e1_f64 * t25721 + 0.22477725215078486977e2_f64 * t146 * t318 * t23503 * t324 + 0.49555929988790549556e3_f64 * t7427 * t8109;
    t25729
}
