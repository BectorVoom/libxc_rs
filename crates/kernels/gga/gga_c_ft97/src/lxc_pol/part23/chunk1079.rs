//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1079/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1079<F: Float>(t1240: F, t2681: F, t1234: F, t4226: F, t230: F, t4977: F, t51: F, t6247: F, t2691: F, t1200: F, t7606: F, t19106: F, t800: F, t19233: F, t287: F, t4092: F) -> (F, F, F, F, F, F, F, F, F) {
    let t69996 = t2681 * t1240;
    let t70038 = t1234 * t4226;
    let t70290 = t230 * t4977;
    let t70456 = t6247 * t51;
    let t70457 = t2691 * t70456;
    let t70497 = t1200 * t7606;
    let t70550 = t800 * t19106;
    let t70671 = t19233 * t287;
    let t70779 = t4092 * t19106;
    (t69996, t70038, t70290, t70456, t70457, t70497, t70550, t70671, t70779)
}
