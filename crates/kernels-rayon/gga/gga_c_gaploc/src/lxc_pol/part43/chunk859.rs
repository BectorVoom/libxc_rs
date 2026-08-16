//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 859/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk859(t2876: f64, t9453: f64, t3159: f64, t12874: f64, t4527: f64, t4614: f64, t204: f64, t41749: f64, t587: f64, t41738: f64, t6710: f64, t6711: f64) -> (f64, f64, f64, f64) {
    let t42296 = t2876 * t9453;
    let t42298 = 0.16683561977530199113e1_f64 * t3159 * t42296;
    let t42305 = 0.36809208915763919689e2_f64 * t4527 * t4614 * t12874;
    let t42309 = 0.18404604457881959845e2_f64 * t587 * t204 * t41749;
    let t42312 = 0.43710935587469654631e2_f64 * t6710 * t6711 * t41738;
    (t42298, t42305, t42309, t42312)
}
