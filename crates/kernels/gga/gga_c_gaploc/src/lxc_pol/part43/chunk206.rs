//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 206/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk206<F: Float>(t188: F, t999: F, t531: F, t993: F, t569: F, t986: F, t568: F, t600: F, t193: F, t557: F, t574: F, t597: F, t902: F, t915: F) -> (F, F, F, F, F, F, F) {
    let t1000 = t188 * t999;
    let t1004 = t531 * t993;
    let t1007 = t569 * t986;
    let t1008 = t568 * t1007;
    let t1012 = t600 * t986;
    let t1013 = t568 * t1012;
    let t1016 = F::cast_from(0.35750489951850426669e0_f64) * t1000 * t193 + F::cast_from(0.29792074959875355558e-1_f64) * t902 - F::cast_from(0.35750489951850426669e0_f64) * t557 * t1004 - F::cast_from(0.23005755572352449806e1_f64) * t574 * t1008 - F::cast_from(0.19171462976960374838e0_f64) * t915 + F::cast_from(0.23005755572352449806e1_f64) * t597 * t1013;
    (t1000, t1004, t1007, t1008, t1012, t1013, t1016)
}
