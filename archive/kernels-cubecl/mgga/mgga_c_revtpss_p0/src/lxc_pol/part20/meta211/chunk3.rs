//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 992/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk992<F: Float>(t10414: F, t117: F, t116: F, t2319: F, t10194: F, t10259: F, t1312: F, t2322: F, t2371: F, t5523: F, t670: F, t2389: F, t705: F) -> (F, F, F, F) {
    let t10415 = t10414 * t117;
    let t10416 = t2319 * t116;
    let t10426 = F::cast_from(2.0_f64) * t10259 * t1312 + F::cast_from(6.0_f64) * t10416 * t670 + F::cast_from(6.0_f64) * t2322 * t2371 + F::cast_from(6.0_f64) * t2371 * t5523 + F::cast_from(6.0_f64) * t10194 + t10415;
    let t10428 = t705 * t2389;
    (t10415, t10416, t10426, t10428)
}
