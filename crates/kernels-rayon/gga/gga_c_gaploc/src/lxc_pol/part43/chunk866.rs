//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 866/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk866(t11392: f64, t3159: f64, t40549: f64, t40555: f64, t40558: f64, t40564: f64, t40567: f64, t40570: f64, t2902: f64, t3145: f64, t4349: f64, t2497: f64, t3366: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t42444 = 0.25025342966295298669e1_f64 * t3159 * t11392;
    let t42455 = 0.11916829983950142223e0_f64 * t40549;
    let t42456 = 0.89376224879626066674e-1_f64 * t40555;
    let t42457 = 0.59584149919750711116e-1_f64 * t40558;
    let t42459 = 0.1022478025437886658e1_f64 * t40564;
    let t42460 = 0.25561950635947166451e1_f64 * t40567;
    let t42461 = 0.29792074959875355558e-1_f64 * t40570;
    let t42470 = 6.0_f64 * t4349 * t2902 * t3145;
    let t42473 = 12.0_f64 * t4349 * t3366 * t2497;
    (t42444, t42455, t42456, t42457, t42459, t42460, t42461, t42470, t42473)
}
