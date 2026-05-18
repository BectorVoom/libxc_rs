//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 903/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk903<F: Float>(t1475: F, t1970: F, t1971: F, t515: F, t866: F, t36769: F, t8443: F, t36924: F, t9082: F, t7255: F, t8447: F, t236: F, t495: F, t5605: F, t7453: F) -> (F, F, F, F, F) {
    let t39605 = t1970 * t1971 * t515 * t1475 * t866;
    let t39607 = t36769 * t8443;
    let t39609 = t36924 * t9082;
    let t39615 = t7255 * t8447;
    let t39620 = t7453 * t1971 * t236 * t5605 * t495;
    (t39605, t39607, t39609, t39615, t39620)
}
