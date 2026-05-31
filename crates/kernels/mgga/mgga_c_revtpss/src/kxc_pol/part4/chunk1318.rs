//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1318/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1318<F: Float>(t3498: F, t5205: F, t1196: F, t12485: F, t1756: F, t3524: F, t3531: F, t5198: F, t12361: F, t5068: F, t12243: F, t5109: F) -> (F, F, F, F, F) {
    let t16639 = t5205 * t3498;
    let t16641 = F::cast_from(0.35089341735807877242e1_f64) * t1196 * t16639;
    let t16642 = t12485 * t1756;
    let t16643 = t16642 * t3524;
    let t16645 = F::cast_from(0.10389515463408878255e3_f64) * t1196 * t16643;
    let t16647 = F::cast_from(0.23392894490538584828e1_f64) * t3531 * t5198;
    let t16649 = F::cast_from(4.0_f64) * t12361 * t5068;
    let t16651 = F::cast_from(0.32163958997385070134e2_f64) * t12243 * t5109;
    (t16641, t16645, t16647, t16649, t16651)
}
