//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 957/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk957<F: Float>(t40567: F, t40570: F, t2902: F, t3145: F, t4349: F, t2497: F, t3366: F, t8045: F, t9260: F, t12862: F, t605: F, t10298: F, t6556: F) -> (F, F, F, F, F, F, F) {
    let t42460 = F::cast_from(0.25561950635947166451e1_f64) * t40567;
    let t42461 = F::cast_from(0.29792074959875355558e-1_f64) * t40570;
    let t42470 = F::cast_from(6.0_f64) * t4349 * t2902 * t3145;
    let t42473 = F::cast_from(12.0_f64) * t4349 * t3366 * t2497;
    let t42475 = F::cast_from(2.0_f64) * t8045 * t9260;
    let t42481 = F::cast_from(6.0_f64) * t4349 * t12862 * t605;
    let t42483 = F::cast_from(4.0_f64) * t6556 * t10298;
    (t42460, t42461, t42470, t42473, t42475, t42481, t42483)
}
