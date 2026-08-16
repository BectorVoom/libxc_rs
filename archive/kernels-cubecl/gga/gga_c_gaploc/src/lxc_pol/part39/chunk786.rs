//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 786/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk786<F: Float>(t12446: F, t12450: F, t123: F, t3689: F, t883: F, t912: F, t587: F, t2488: F, t2487: F, t12911: F, t12912: F, t12916: F, t12921: F, t12924: F, t12928: F, t12931: F, t12932: F, t12935: F, t12936: F, t12937: F) -> (F, F, F, F, F) {
    let t13775 = F::cast_from(0.63904876589867916128e-1_f64) * t12446;
    let t13776 = F::cast_from(0.63904876589867916128e-1_f64) * t12450;
    let t13777 = t3689 * t123;
    let t13778 = t13777 * t883;
    let t13779 = t912 * t13778;
    let t13780 = t587 * t13779;
    let t13782 = t2488 * t13778;
    let t13783 = t2487 * t13782;
    let t13785 = t12911 + F::cast_from(0.35750489951850426669e0_f64) * t12912 - F::cast_from(0.69017266717057349418e1_f64) * t12916 - t12921 + t12924 - t12928 - t13775 + t13776 - t12931 + t12932 - F::cast_from(0.19171462976960374838e0_f64) * t13780 + F::cast_from(0.19171462976960374838e0_f64) * t13783 - t12935 + t12936 + t12937;
    (t13777, t13778, t13779, t13782, t13785)
}
