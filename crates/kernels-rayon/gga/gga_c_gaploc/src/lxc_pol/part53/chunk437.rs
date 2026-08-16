//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 437/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk437(t1959: f64, t744: f64, t746: f64, t304: f64) -> (f64, f64, f64, f64) {
    let t5552 = t744 * t1959;
    let t5557 = t746 * t746;
    let t5558 = 1.0_f64 / t5557;
    let t5559 = t304 * t5558;
    (t5552, t5557, t5558, t5559)
}
