//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 968/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk968<F: Float>(t102: F, t4880: F, t4859: F, t23: F, t821: F, t6: F, t107: F, t4866: F, t2621: F, t9: F, t7: F, t118: F, t4882: F, t1737: F, t2471: F, t1742: F, t2475: F) -> (F, F, F, F, F, F, F, F) {
    let t13577 = t102 * t4880;
    let t13578 = t13577 * t4859;
    let t13581 = 1.0 / t23 / t821;
    let t13582 = t6 * t13581;
    let t13583 = t107 * t13582;
    let t13584 = t13583 * t4866;
    let t13587 = 1.0 / t9 / t2621;
    let t13588 = t7 * t13587;
    let t13589 = t118 * t13588;
    let t13590 = t13589 * t4882;
    let t13592 = t2471 * t1737;
    let t13594 = t2475 * t1742;
    (t13577, t13578, t13583, t13584, t13589, t13590, t13592, t13594)
}
