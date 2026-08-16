//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 867/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk867<F: Float>(t45423: F, t6066: F, t6111: F, t10914: F, t10915: F, t326: F, t45369: F, t825: F, t13588: F, t549: F, t11757: F, t9823: F) -> (F, F, F, F, F) {
    let t45426 = F::cast_from(0.42900587942220512003e1_f64) * t6111 * t6066 * t45423;
    let t45429 = F::cast_from(0.21450293971110256001e1_f64) * t10914 * t10915 * t45423;
    let t45432 = F::cast_from(0.18404604457881959845e2_f64) * t825 * t326 * t45369;
    let t45437 = t6111 * t549 * t13588;
    let t45438 = F::cast_from(0.59584149919750711116e-1_f64) * t45437;
    let t45440 = F::cast_from(0.35750489951850426669e0_f64) * t9823 * t11757;
    (t45426, t45429, t45432, t45438, t45440)
}
