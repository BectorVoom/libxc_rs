//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 420/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk420<F: Float>(t103: F, t8: F, t417: F, t62: F, t1234: F, t89: F, t1238: F, t142: F, t1246: F, t458: F, t462: F, t153: F) -> (F, F, F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t3816 = t8 * t103;
    let t3831 = t62 * t417;
    let t4061 = t1234 * t89;
    let t4072 = t1238 * t1238;
    let t4074 = F::cast_from(1.0_f64) / t4072 / t142;
    let t4077 = pi * t1246 * t458;
    let t4080 = t462 * t462;
    let t4081 = F::cast_from(1.0_f64) / t4080;
    let t4082 = t153 * t4081;
    (t3816, t3831, t4061, t4072, t4074, t4077, t4080, t4081, t4082)
}
