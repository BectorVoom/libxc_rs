//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 493/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk493<F: Float>(t2641: F, t243: F, t803: F, t40: F, t218: F, t771: F, t777: F, t779: F, t220: F, t760: F, t271: F, t680: F, t690: F) -> (F, F, F, F, F) {
    let t2642 = F::new(12.0) * t2641;
    let t2643 = t243 * t803;
    let t2644 = t40 * t2643;
    let t2654 = t777 * t771 * t779 * t218;
    let t2655 = F::cast_from(0.48245938496077605201e2_f64) * t2654;
    let t2657 = t760 * t220 * t771;
    let t2658 = F::new(6.0) * t2657;
    let t2660 = t680 * t690 * t271;
    (t2642, t2644, t2655, t2658, t2660)
}
