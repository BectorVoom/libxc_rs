//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 700/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk700<F: Float>(t1928: F, t2035: F, t6931: F, t127: F, t2022: F, t616: F, t2034: F, t2010: F, t623: F, t2013: F, t56: F, t658: F, t111: F, t5: F, t629: F, t6856: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t6932 = t2035 * t1928;
    let t6933 = t6931 * t6932;
    let t6936 = t2022 * t127;
    let t6937 = t6936 * t616;
    let t6938 = t2034 * t6937;
    let t6941 = t623 * t2010;
    let t6942 = t6941 * t2013;
    let t6944 = t56 * t658;
    let t6945 = t111 * t6944;
    let t6947 = t629 * t5 * t6856;
    (t6932, t6933, t6936, t6937, t6938, t6941, t6942, t6944, t6945, t6947)
}
