//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 866/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk866<F: Float>(t11392: F, t3159: F, t40549: F, t40555: F, t40558: F, t40564: F, t40567: F, t40570: F, t2902: F, t3145: F, t4349: F, t2497: F, t3366: F) -> (F, F, F, F, F, F, F, F, F) {
    let t42444 = F::cast_from(0.25025342966295298669e1_f64) * t3159 * t11392;
    let t42455 = F::cast_from(0.11916829983950142223e0_f64) * t40549;
    let t42456 = F::cast_from(0.89376224879626066674e-1_f64) * t40555;
    let t42457 = F::cast_from(0.59584149919750711116e-1_f64) * t40558;
    let t42459 = F::cast_from(0.1022478025437886658e1_f64) * t40564;
    let t42460 = F::cast_from(0.25561950635947166451e1_f64) * t40567;
    let t42461 = F::cast_from(0.29792074959875355558e-1_f64) * t40570;
    let t42470 = F::new(6.0) * t4349 * t2902 * t3145;
    let t42473 = F::new(12.0) * t4349 * t3366 * t2497;
    (t42444, t42455, t42456, t42457, t42459, t42460, t42461, t42470, t42473)
}
