//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1080/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1080(t3394: f64, t486: f64, t4144: f64, t987: f64, t4245: f64, t4398: f64, t8410: f64, t1: f64, t25760: f64, t1415: f64, t1519: f64, t2876: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t25893 = t3394 * t486;
    let t25955 = t987 * t4144;
    let t26011 = t987 * t4245;
    let t26122 = t4398 * t8410;
    let t26126 = t25760 * t1;
    let t26127 = t1415 * t26126;
    let t26244 = t2876 * t1519;
    (t25893, t25955, t26011, t26122, t26126, t26127, t26244)
}
