//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 805/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk805<F: Float>(t32744: F, t9824: F, t10924: F, t1980: F, t13072: F, t32969: F, t10867: F, t41511: F, t25070: F, t7427: F, t9438: F, t41408: F) -> (F, F, F, F, F, F) {
    let t43914 = t32744 * t9824;
    let t43917 = t1980 * t10924 * t9824;
    let t43925 = t32969 * t13072;
    let t43927 = t10867 * t41511;
    let t43930 = t7427 * t9438 * t25070;
    let t43994 = F::cast_from(0.19171462976960374838e0_f64) * t41408;
    (t43914, t43917, t43925, t43927, t43930, t43994)
}
