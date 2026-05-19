//! GGA_C_GAPLOC lxc pol — lxc_pol part 53 (v4rhosigma3_18) CSE chunk 573/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part53_v4rhosigma3_18_chunk573<F: Float>(t10027: F, t10065: F, t10102: F, t9827: F, t9867: F, t9907: F, t9938: F, t9980: F, t2440: F, t988: F, t2268: F, t2756: F, t894: F) -> (F, F, F) {
    let t10105 = t9827 + t9867 + t9907 + t9938 + t9980 + t10027 + t10065 + t10102;
    let t10113 = t2440 * t988;
    let t10115 = F::cast_from(0.28455006635676149599e-1_f64) * t2268 * t10113;
    let t10116 = t894 * t2756;
    (t10105, t10115, t10116)
}
