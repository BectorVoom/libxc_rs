//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 966/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk966(t313: f64, t3470: f64, t43246: f64, t10789: f64, t769: f64, t10627: f64, t2530: f64) -> (f64, f64, f64) {
    let t43481 = t313 * t43246 * t3470;
    let t43484 = t769 * t10789 * t3470;
    let t43486 = t10627 * t2530;
    (t43481, t43484, t43486)
}
