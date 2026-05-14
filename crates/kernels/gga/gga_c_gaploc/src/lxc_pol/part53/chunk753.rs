//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 753/923 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk753<F: Float>(t40549: F, t40555: F, t40558: F, t40564: F, t40567: F, t40570: F, t2902: F, t3145: F, t4349: F, t2497: F, t3366: F, t8045: F, t9260: F, t12862: F, t605: F, t10298: F, t6556: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t42455 = 0.11916829983950142223e0 * t40549;
    let t42456 = 0.89376224879626066674e-1 * t40555;
    let t42457 = 0.59584149919750711116e-1 * t40558;
    let t42459 = 0.1022478025437886658e1 * t40564;
    let t42460 = 0.25561950635947166451e1 * t40567;
    let t42461 = 0.29792074959875355558e-1 * t40570;
    let t42470 = 6.0 * t4349 * t2902 * t3145;
    let t42473 = 12.0 * t4349 * t3366 * t2497;
    let t42475 = 2.0 * t8045 * t9260;
    let t42481 = 6.0 * t4349 * t12862 * t605;
    let t42483 = 4.0 * t6556 * t10298;
    (t42455, t42456, t42457, t42459, t42460, t42461, t42470, t42473, t42475, t42481, t42483)
}
