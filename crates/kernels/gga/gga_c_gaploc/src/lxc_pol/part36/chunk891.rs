//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 891/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk891<F: Float>(t2902: F, t3145: F, t4349: F, t2497: F, t3366: F, t8045: F, t9260: F, t13001: F, t1382: F, t605: F, t12862: F, t10298: F, t6556: F) -> (F, F, F, F, F, F) {
    let t42470 = F::cast_from(6.0_f64) * t4349 * t2902 * t3145;
    let t42473 = F::cast_from(12.0_f64) * t4349 * t3366 * t2497;
    let t42475 = F::cast_from(2.0_f64) * t8045 * t9260;
    let t42478 = F::cast_from(2.0_f64) * t1382 * t13001 * t605;
    let t42481 = F::cast_from(6.0_f64) * t4349 * t12862 * t605;
    let t42483 = F::cast_from(4.0_f64) * t6556 * t10298;
    (t42470, t42473, t42475, t42478, t42481, t42483)
}
