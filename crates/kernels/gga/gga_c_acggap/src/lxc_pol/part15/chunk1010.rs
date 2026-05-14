//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1010/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1010<F: Float>(t1967: F, t9531: F, t1901: F, t7614: F, t30468: F, t6144: F, t7433: F, t9758: F, t34481: F, t5855: F, t5859: F, t8511: F, t2001: F, t5681: F, t6106: F, t6110: F) -> (F, F, F, F, F, F, F, F, F) {
    let t39946 = t1967 * t9531;
    let t39948 = t7614 * t1901;
    let t39950 = t30468 * t6144;
    let t39952 = t7433 * t9758;
    let t39962 = t34481 * t5855;
    let t39965 = t8511 * t5859;
    let t39967 = t2001 * t5681;
    let t39969 = t2001 * t6106;
    let t39971 = t2001 * t6110;
    (t39946, t39948, t39950, t39952, t39962, t39965, t39967, t39969, t39971)
}
