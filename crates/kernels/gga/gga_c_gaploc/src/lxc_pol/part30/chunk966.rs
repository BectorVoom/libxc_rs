//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 966/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk966<F: Float>(t10513: F, t10526: F, t10525: F, t19: F, t4524: F, t60: F, t584: F) -> (F, F, F, F, F) {
    let t10527 = t10526 * t10513;
    let t10529 = F::new(0.21450293971110256001e1) * t10525 * t10527;
    let t10530 = t4524 * t19;
    let t10531 = t10530 * t60;
    let t10532 = t584 * t10531;
    (t10527, t10529, t10530, t10531, t10532)
}
