//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 659/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk659(t1: f64, t3720: f64, t106: f64, t316: f64, t3732: f64, t773: f64, t835: f64, t723: f64, t1457: f64, t2089: f64, t1445: f64, t325: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t12205 = t3720 * t1;
    let t12206 = t12205 * t106;
    let t12207 = t12206 * t316;
    let t12210 = t773 * t3732;
    let t12213 = t835 * t3720;
    let t12214 = t12213 * t723;
    let t12215 = t1457 * t12214;
    let t12218 = t2089 * t3720;
    let t12219 = t12218 * t723;
    let t12220 = t1445 * t12219;
    let t12223 = t325 * t3720;
    (t12207, t12210, t12213, t12214, t12215, t12220, t12223)
}
