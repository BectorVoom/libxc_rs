//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 844/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk844<F: Float>(t12968: F, t34471: F, t34286: F, t10615: F, t40186: F, t41809: F, t475: F) -> (F, F, F, F) {
    let t41947 = t34471 * t12968;
    let t41949 = t34286 * t12968;
    let t41951 = t10615 * t40186;
    let t41965 = t41809 * t475;
    (t41947, t41949, t41951, t41965)
}
