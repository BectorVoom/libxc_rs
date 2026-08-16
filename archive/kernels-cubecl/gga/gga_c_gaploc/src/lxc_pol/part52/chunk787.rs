//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 787/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk787<F: Float>(t10615: F, t1423: F, t3129: F, t40377: F, t2890: F, t9267: F, t9278: F, t20671: F, t31047: F, t34814: F, t26984: F, t9294: F) -> (F, F, F, F, F) {
    let t42156 = t10615 * t1423 * t3129;
    let t42170 = F::cast_from(0.19171462976960374838e0_f64) * t40377;
    let t42183 = t9267 * t2890 * t9278;
    let t42187 = t31047 * t20671 * t34814;
    let t42189 = t26984 * t9294;
    (t42156, t42170, t42183, t42187, t42189)
}
