//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 653/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk653<F: Float>(t26867: F, t26922: F, t26976: F, t27196: F, t27238: F, t27285: F, t27312: F, t27403: F, t609: F, t6708: F, t160: F, t27391: F, t24081: F, t3424: F, t24080: F, t1360: F, t378: F) -> (F, F, F, F, F, F) {
    let t27406 = t26867 + t26922 + t26976 + t27196 + t27238 + t27285 + t27312 + t27403;
    let t27411 = t6708 * t609;
    let t27414 = t27391 * t160;
    let t27416 = t24081 * t3424;
    let t27417 = t24080 * t27416;
    let t27420 = t378 * t1360;
    (t27406, t27411, t27414, t27416, t27417, t27420)
}
