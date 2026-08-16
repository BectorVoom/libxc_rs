//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1144/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1144(t2321: f64, t28438: f64, t9285: f64, t2487: f64, t6985: f64, t9278: f64, t18067: f64, t9558: f64, t20513: f64, t2365: f64, t4391: f64, t20521: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t30733 = t28438 * t2321;
    let t30735 = 0.11916829983950142223e0_f64 * t9285 * t30733;
    let t30751 = t2487 * t6985 * t9278;
    let t30754 = 0.11916829983950142223e0_f64 * t18067 * t9558;
    let t30757 = 0.11916829983950142223e0_f64 * t4391 * t2365 * t20513;
    let t30760 = 0.59584149919750711116e-1_f64 * t4391 * t2365 * t20521;
    (t30733, t30735, t30751, t30754, t30757, t30760)
}
