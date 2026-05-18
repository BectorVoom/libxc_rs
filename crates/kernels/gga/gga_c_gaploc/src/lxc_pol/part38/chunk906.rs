//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 906/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk906<F: Float>(t37218: F, t955: F, t11798: F, t9972: F, t13609: F, t36738: F, t13647: F, t4614: F, t813: F, t11757: F, t2714: F, t2718: F) -> (F, F, F, F, F, F) {
    let t45383 = F::new(0.35750489951850426669e0) * t955 * t37218;
    let t45385 = F::new(0.10725146985555128001e1) * t11798 * t9972;
    let t45387 = F::new(0.42900587942220512003e1) * t36738 * t13609;
    let t45390 = F::new(0.61348681526273199483e1) * t813 * t4614 * t13647;
    let t45392 = F::new(0.35750489951850426669e0) * t2714 * t11757;
    let t45394 = F::new(0.35750489951850426669e0) * t2718 * t11757;
    (t45383, t45385, t45387, t45390, t45392, t45394)
}
