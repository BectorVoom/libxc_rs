//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 309/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk309(t2482: f64, t912: f64, t587: f64, t1535: f64, t60: f64, t584: f64) -> (f64, f64, f64) {
    let t2483 = t912 * t2482;
    let t2484 = t587 * t2483;
    let t2486 = t1535 * t60;
    let t2487 = t584 * t2486;
    (t2484, t2486, t2487)
}
