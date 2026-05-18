//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 415/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk415<F: Float>(t2360: F, t327: F, t326: F, t2400: F, t1537: F, t947: F, t1546: F, t89: F, t921: F) -> (F, F, F, F, F, F) {
    let t2928 = t327 * t2360;
    let t2937 = t326 * t326;
    let t2938 = F::new(1.0) / t2937;
    let t2946 = F::new(0.19257444444444444444e0) * t2400;
    let t2976 = t1537 * t947;
    let t2981 = t89 * t1546 * t921;
    (t2928, t2937, t2938, t2946, t2976, t2981)
}
