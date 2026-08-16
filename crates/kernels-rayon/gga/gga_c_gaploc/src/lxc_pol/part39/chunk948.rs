//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 948/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk948(t1445: f64, t3116: f64, t574: f64, t7980: f64, t2778: f64, t9127: f64, t2876: f64, t9453: f64, t3159: f64, t12874: f64, t4527: f64, t4614: f64) -> (f64, f64, f64, f64) {
    let t42288 = 0.46011511144704899612e1_f64 * t574 * t1445 * t7980 * t3116;
    let t42292 = 0.46011511144704899612e1_f64 * t574 * t1445 * t2778 * t9127;
    let t42296 = t2876 * t9453;
    let t42298 = 0.16683561977530199113e1_f64 * t3159 * t42296;
    let t42305 = 0.36809208915763919689e2_f64 * t4527 * t4614 * t12874;
    (t42288, t42292, t42298, t42305)
}
