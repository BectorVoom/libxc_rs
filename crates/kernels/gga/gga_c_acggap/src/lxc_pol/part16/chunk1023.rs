//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1023/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1023<F: Float>(t2068: F, t4680: F, t9592: F, t7839: F, t9583: F, t9582: F, t1181: F, t1839: F, t360: F, t604: F, t27011: F, t7351: F, t7575: F, t26956: F, t7564: F, t8600: F) -> (F, F, F, F, F, F) {
    let t39985 = t2068 * t4680 * t9592;
    let t39987 = t7839 * t9583;
    let t39990 = t2068 * t4680 * t9582;
    let t39995 = t2068 * t1181 * t604 * t1839 * t360;
    let t39999 = t7575 * t1181 * t7351 * t27011;
    let t40003 = t7564 * t1181 * t8600 * t26956;
    (t39985, t39987, t39990, t39995, t39999, t40003)
}
