//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 414/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk414(t1445: f64, t3503: f64, t2087: f64, t3111: f64, t3114: f64, t3330: f64, t3333: f64, t3335: f64) -> (f64, f64, f64) {
    let t3504 = t1445 * t3503;
    let t3506 = 0.69017266717057349418e1_f64 * t2087 * t3504;
    let t3689 = t3335 + t3111 + t3330 - t3333 - t3114;
    (t3504, t3506, t3689)
}
