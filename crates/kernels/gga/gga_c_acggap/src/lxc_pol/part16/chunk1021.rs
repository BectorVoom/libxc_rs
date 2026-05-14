//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1021/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1021<F: Float>(t34481: F, t5855: F, t5859: F, t8511: F, t2001: F, t5681: F, t6106: F, t6110: F, t1896: F, t7605: F, t1992: F, t6847: F, t7585: F, t7586: F, t1181: F, t2068: F, t25727: F, t604: F) -> (F, F, F, F, F, F, F, F) {
    let t39962 = t34481 * t5855;
    let t39965 = t8511 * t5859;
    let t39967 = t2001 * t5681;
    let t39969 = t2001 * t6106;
    let t39971 = t2001 * t6110;
    let t39973 = t7605 * t1896;
    let t39977 = t7585 * t7586 * t1992 * t6847;
    let t39981 = t2068 * t1181 * t604 * t25727;
    (t39962, t39965, t39967, t39969, t39971, t39973, t39977, t39981)
}
