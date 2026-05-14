//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 398/884 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk398<F: Float>(t103: F, t8: F, t417: F, t62: F, t1234: F, t89: F, t1238: F, t142: F, t1246: F, t458: F, t462: F, t153: F, t1564: F, t169: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3816 = t8 * t103;
    let t3831 = t62 * t417;
    let t4061 = t1234 * t89;
    let t4072 = t1238 * t1238;
    let t4074 = 1.0 / t4072 / t142;
    let t4077 = M_PI * t1246 * t458;
    let t4080 = t462 * t462;
    let t4081 = 1.0 / t4080;
    let t4082 = t153 * t4081;
    let t4085 = t4074 * M_PI * t458;
    let t4130 = t169 * t1564;
    (t3816, t3831, t4061, t4072, t4074, t4077, t4080, t4081, t4082, t4085, t4130)
}
