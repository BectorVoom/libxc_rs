//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 143/1012 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk143(t403: f64, t406: f64, t408: f64, t413: f64, t90: f64, t257: f64, t260: f64, t266: f64, t657: f64, t667: f64, t670: f64, t61: f64, t63: f64) -> (f64, f64, f64) {
    let t677 = 0.77371026992393176896e-2_f64 * t90 - 0.2499945e-2_f64 * t403 + 0.604634375e-3_f64 * t406 - 0.20417003743104289064e-4_f64 * t408 + 0.20205871875e-5_f64 * t413;
    let t679 = -0.10636476373080147432e-2_f64 * t90 * t257 - 0.21272952746160294864e-2_f64 * t657 * t667 - t670 * t266 - t260 * t677;
    let t681 = t61 * t63 * t679;
    (t677, t679, t681)
}
