//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 890/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk890(t44712: f64, t5241: f64, t5640: f64, t590: f64, t43400: f64, t43403: f64, t43407: f64, t2617: f64, t3621: f64, t7803: f64, t43412: f64, t43416: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t45192 = 0.15337170381568299871e1_f64 * t5640 * t5241 * t44712 * t590;
    let t45193 = 0.30674340763136599742e1_f64 * t43400;
    let t45194 = 0.20705180015117204825e2_f64 * t43403;
    let t45195 = 0.92023022289409799224e1_f64 * t43407;
    let t45197 = t7803 * t3621 * t2617;
    let t45199 = 0.15337170381568299871e1_f64 * t43412;
    let t45200 = 0.15337170381568299871e1_f64 * t43416;
    (t45192, t45193, t45194, t45195, t45197, t45199, t45200)
}
