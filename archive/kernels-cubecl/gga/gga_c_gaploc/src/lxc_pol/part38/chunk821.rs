//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 821/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk821<F: Float>(t13064: F, t2684: F, t7354: F, t10867: F, t1423: F, t3247: F, t13077: F, t28439: F, t32744: F, t9824: F, t10924: F, t1980: F) -> (F, F, F, F, F) {
    let t43904 = t2684 * t7354 * t13064;
    let t43907 = t10867 * t1423 * t3247;
    let t43912 = t13077 * t28439;
    let t43914 = t32744 * t9824;
    let t43917 = t1980 * t10924 * t9824;
    (t43904, t43907, t43912, t43914, t43917)
}
