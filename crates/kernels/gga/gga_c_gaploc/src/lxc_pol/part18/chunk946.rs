//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 946/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk946<F: Float>(t10295: F, t4349: F, t2902: F, t921: F, t1382: F, t1016: F, t2497: F, t1377: F, t3418: F, t605: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10296 = t4349 * t10295;
    let t10297 = F::cast_from(6.0_f64) * t10296;
    let t10298 = t2902 * t921;
    let t10299 = t1382 * t10298;
    let t10300 = F::cast_from(2.0_f64) * t10299;
    let t10301 = t1016 * t2497;
    let t10302 = t1382 * t10301;
    let t10303 = F::cast_from(2.0_f64) * t10302;
    let t10304 = t1377 * t3418;
    let t10305 = t3418 * t605;
    let t10306 = t1382 * t10305;
    let t10307 = F::cast_from(2.0_f64) * t10306;
    (t10296, t10297, t10298, t10299, t10300, t10301, t10302, t10303, t10304, t10305, t10306, t10307)
}
