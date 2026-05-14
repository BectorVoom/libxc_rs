//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 875/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk875<F: Float>(t3915: F, t5371: F, t12203: F, t3044: F, t448: F, t556: F, t5360: F, t872: F, t5379: F, t868: F, t14690: F, t557: F, t1308: F, t3909: F, t3918: F, t5385: F) -> (F, F, F, F, F, F, F) {
    let t15159 = t5371 * t3915;
    let t15164 = t12203 * t448 * t556 * t3044;
    let t15175 = t5360 * t872;
    let t15177 = t868 * t5379;
    let t15179 = t14690 * t557;
    let t15184 = t1308 * t3909;
    let t15190 = t12203 * t5385 * t3918;
    (t15159, t15164, t15175, t15177, t15179, t15184, t15190)
}
