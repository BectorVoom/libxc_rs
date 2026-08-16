//! GGA_C_GAPLOC lxc pol — lxc_pol part 39 (v4rhosigma3_4) CSE chunk 786/1217 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part39_v4rhosigma3_4_chunk786(t12446: f64, t12450: f64, t123: f64, t3689: f64, t883: f64, t912: f64, t587: f64, t2488: f64, t2487: f64, t12911: f64, t12912: f64, t12916: f64, t12921: f64, t12924: f64, t12928: f64, t12931: f64, t12932: f64, t12935: f64, t12936: f64, t12937: f64) -> (f64, f64, f64, f64, f64) {
    let t13775 = 0.63904876589867916128e-1_f64 * t12446;
    let t13776 = 0.63904876589867916128e-1_f64 * t12450;
    let t13777 = t3689 * t123;
    let t13778 = t13777 * t883;
    let t13779 = t912 * t13778;
    let t13780 = t587 * t13779;
    let t13782 = t2488 * t13778;
    let t13783 = t2487 * t13782;
    let t13785 = t12911 + 0.35750489951850426669e0_f64 * t12912 - 0.69017266717057349418e1_f64 * t12916 - t12921 + t12924 - t12928 - t13775 + t13776 - t12931 + t12932 - 0.19171462976960374838e0_f64 * t13780 + 0.19171462976960374838e0_f64 * t13783 - t12935 + t12936 + t12937;
    (t13777, t13778, t13779, t13782, t13785)
}
