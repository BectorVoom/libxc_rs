//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 819/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk819<F: Float>(t9148: F, t9375: F, t9401: F, t9445: F, t105: F, t469: F, t8040: F, t9089: F, t3952: F, t642: F, t9098: F, t1713: F) -> (F, F, F, F, F, F, F) {
    let t9447 = t9148 + t9375 + t9401 + t9445;
    let t9448 = t105 * t9447;
    let t9449 = t9448 * t469;
    let t9455 = t8040 * t9089;
    let t9460 = t642 * t3952;
    let t9461 = t9460 * t9098;
    let t9469 = t469 * t1713;
    (t9447, t9448, t9449, t9455, t9460, t9461, t9469)
}
