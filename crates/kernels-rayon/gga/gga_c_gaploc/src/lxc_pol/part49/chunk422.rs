//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 422/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk422(t313: f64, t3464: f64, t2976: f64, t959: f64, t1645: f64, t948: f64) -> (f64, f64, f64) {
    let t3465 = t313 * t3464;
    let t3468 = t2976 * t959;
    let t3469 = 0.14896037479937677779e-1_f64 * t3468;
    let t3470 = t1645 * t948;
    (t3465, t3469, t3470)
}
