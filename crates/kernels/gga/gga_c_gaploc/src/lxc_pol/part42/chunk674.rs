//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 674/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk674<F: Float>(t1233: F, t157: F, t874: F, t9439: F, t9438: F, t587: F, t9448: F, t2487: F, t12381: F, t286: F, t708: F, t712: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t12411 = F::new(1.0) / t1233;
    let t12412 = t157 * t12411;
    let t12444 = t9439 * t874;
    let t12445 = t9438 * t12444;
    let t12446 = t587 * t12445;
    let t12448 = t9448 * t874;
    let t12449 = t9438 * t12448;
    let t12450 = t2487 * t12449;
    let t12555 = t12381 * t286 * t708;
    let t12557 = pi * t712;
    (t12411, t12412, t12444, t12445, t12446, t12448, t12449, t12450, t12555, t12557)
}
