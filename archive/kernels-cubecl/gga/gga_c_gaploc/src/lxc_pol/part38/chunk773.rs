//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 773/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk773<F: Float>(t145: F, t39622: F, t459: F, t12385: F, t2281: F, t1246: F, t135: F, t4074: F, t458: F, t9105: F, t1234: F, t3096: F) -> (F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t39624 = t39622 * t145 * t459;
    let t39626 = t2281 * t12385;
    let t39632 = t9105 * t4074 * pi * t1246 * t135 * t458;
    let t39635 = F::cast_from(1.0_f64) / t1234 / t3096;
    (t39624, t39626, t39632, t39635)
}
