//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 725/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk725(t11159: f64, t242: f64, t168: f64, t3609: f64, t703: f64, t163: f64, t169: f64, t299: f64, t3569: f64, t1: f64, t3: f64, t3379: f64) -> (f64, f64, f64, f64) {
    let t11160 = t11159 * t242;
    let t11166 = t168 * t703 * t3609;
    let t11187 = t169 * t299 * t3569 * t163;
    let t11190 = t3379 * t1 * t3;
    (t11160, t11166, t11187, t11190)
}
