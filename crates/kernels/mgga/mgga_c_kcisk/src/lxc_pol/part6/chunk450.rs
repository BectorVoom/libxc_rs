//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 450/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk450<F: Float>(t382: F, t442: F, t143: F, t1055: F, t142: F, t179: F, t139: F) -> (F, F, F, F, F) {
    let t3485 = t382 * t442;
    let t3499 = F::new(2.0) * t143;
    let t3500 = F::new(2.0) * t1055;
    let t3516 = t179 * t142;
    let t3517 = t139 * t3516;
    (t3485, t3499, t3500, t3516, t3517)
}
