//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1011/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1011<F: Float>(t1896: F, t7605: F, t1992: F, t6847: F, t7585: F, t7586: F, t1181: F, t2068: F, t25727: F, t604: F, t4680: F, t9592: F, t7839: F, t9583: F, t9582: F, t1839: F, t360: F) -> (F, F, F, F, F, F, F) {
    let t39973 = t7605 * t1896;
    let t39977 = t7585 * t7586 * t1992 * t6847;
    let t39981 = t2068 * t1181 * t604 * t25727;
    let t39985 = t2068 * t4680 * t9592;
    let t39987 = t7839 * t9583;
    let t39990 = t2068 * t4680 * t9582;
    let t39995 = t2068 * t1181 * t604 * t1839 * t360;
    (t39973, t39977, t39981, t39985, t39987, t39990, t39995)
}
