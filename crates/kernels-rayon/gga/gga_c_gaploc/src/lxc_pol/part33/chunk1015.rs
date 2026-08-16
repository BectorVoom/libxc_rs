//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1015/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1015(t12214: f64, t1457: f64, t2089: f64, t3720: f64, t723: f64, t1445: f64, t325: f64) -> (f64, f64, f64, f64, f64) {
    let t12215 = t1457 * t12214;
    let t12218 = t2089 * t3720;
    let t12219 = t12218 * t723;
    let t12220 = t1445 * t12219;
    let t12223 = t325 * t3720;
    (t12215, t12218, t12219, t12220, t12223)
}
