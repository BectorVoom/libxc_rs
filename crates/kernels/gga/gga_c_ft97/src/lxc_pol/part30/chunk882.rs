//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 882/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk882<F: Float>(t1501: F, t7124: F, t2843: F, t6961: F, t7150: F, t1091: F, t1477: F, t666: F, t461: F, t7087: F, t231: F, t7021: F) -> (F, F, F, F, F, F, F) {
    let t36068 = t1501 * t7124;
    let t36069 = t2843 * t36068;
    let t36071 = t6961 * t7150;
    let t36074 = t1477 * t1091;
    let t36075 = t666 * t36074;
    let t36080 = t461 * t7087;
    let t36086 = t231 * t7021;
    (t36068, t36069, t36071, t36074, t36075, t36080, t36086)
}
