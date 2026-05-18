//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 867/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk867<F: Float>(t8045: F, t9260: F, t12862: F, t4349: F, t605: F, t10298: F, t6556: F, t12856: F, t17288: F, t2801: F, t31428: F, t1016: F, t1382: F, t9588: F) -> (F, F, F, F, F, F) {
    let t42475 = F::new(2.0) * t8045 * t9260;
    let t42481 = F::new(6.0) * t4349 * t12862 * t605;
    let t42483 = F::new(4.0) * t6556 * t10298;
    let t42485 = F::new(6.0) * t17288 * t12856;
    let t42487 = F::new(2.0) * t31428 * t2801;
    let t42491 = F::new(2.0) * t1382 * t1016 * t9588;
    (t42475, t42481, t42483, t42485, t42487, t42491)
}
