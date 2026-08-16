//! GGA_C_GAPC lxc pol — lxc_pol part 29 (v4rho2sigma2_8) CSE chunk 1180/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part29_v4rho2sigma2_8_chunk1180<F: Float>(t34550: F, t34553: F, t34555: F, t34557: F, t34560: F, t34563: F, t34565: F, t34567: F, t34570: F, t34573: F, t34576: F, t3144: F, t34372: F) -> (F, F) {
    let t34578 = F::cast_from(0.45289771048911752714e-7_f64) * t34550 + F::cast_from(0.67530371184977617164e-6_f64) * t34553 + F::cast_from(0.67530371184977617164e-6_f64) * t34555 + F::cast_from(0.33765185592488808582e-6_f64) * t34557 + F::cast_from(0.52838066223730378166e-7_f64) * t34560 - F::cast_from(0.58366874983904959946e-8_f64) * t34563 - F::cast_from(0.6629778687778673199e-7_f64) * t34565 - F::cast_from(0.33148893438893365995e-7_f64) * t34567 + F::cast_from(0.687148483626368822e-6_f64) * t34570 - F::cast_from(0.33765185592488808582e-6_f64) * t34573 - F::cast_from(0.45020247456651744776e-7_f64) * t34576;
    let t34582 = t34372 * t3144;
    (t34578, t34582)
}
