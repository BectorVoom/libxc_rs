//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1407/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1407<F: Float>(t1037: F, t1056: F, t59103: F, t59116: F, t59132: F, t59147: F, t17427: F, t34422: F, t58812: F, t58820: F, t58822: F, t58834: F, t58836: F, t58864: F, t58884: F, t58888: F, t59086: F, t59088: F) -> (F, F, F) {
    let t59152 = F::new(1.0) * t1037 * (t59103 + t59116 + t59132 + t59147) * t1056;
    let t59154 = F::cast_from(0.20690005882282467367e4_f64) * t34422 * t17427;
    let t59155 = t58812 - t58820 - t58822 + t58834 + t58836 + t58864 + t58884 - t58888 - t59086 + t59088 + t59152 + t59154;
    (t59152, t59154, t59155)
}
