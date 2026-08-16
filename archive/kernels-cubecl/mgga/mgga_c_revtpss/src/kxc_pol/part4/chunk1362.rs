//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1362/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1362<F: Float>(t1042: F, t17203: F, t3172: F, t5298: F, t3711: F, t1469: F, t3568: F, t5296: F, t5278: F, t1250: F, t17170: F, t482: F) -> (F, F, F, F, F) {
    let t17204 = t1042 * t17203;
    let t17209 = t3172 * t5298;
    let t17211 = F::cast_from(0.19055119163586549765e-3_f64) * t3711 * t17209;
    let t17212 = t1469 * t3568;
    let t17213 = t5296 * t17212;
    let t17214 = t1042 * t17213;
    let t17217 = t3172 * t5278;
    let t17219 = F::cast_from(0.19055119163586549765e-3_f64) * t3711 * t17217;
    let t17221 = t482 * t17170 * t1250;
    (t17204, t17211, t17214, t17219, t17221)
}
