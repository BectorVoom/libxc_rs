//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 713/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk713<F: Float>(t13096: F, t314: F, t313: F, t739: F, t531: F, t808: F, t568: F, t836: F, t12693: F, t12697: F, t12699: F, t12701: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t13097 = t314 * t13096;
    let t13098 = t313 * t13097;
    let t13101 = t739 * t13096;
    let t13102 = t531 * t13101;
    let t13105 = t808 * t13096;
    let t13106 = t568 * t13105;
    let t13109 = t836 * t13096;
    let t13110 = t568 * t13109;
    let t13113 = F::new(0.63904876589867916127e-1) * t12693;
    let t13114 = F::new(0.29792074959875355558e-1) * t12697;
    let t13115 = F::new(0.29792074959875355558e-1) * t12699;
    let t13116 = F::new(0.29792074959875355558e-1) * t12701;
    (t13097, t13098, t13101, t13102, t13105, t13106, t13109, t13110, t13113, t13114, t13115, t13116)
}
