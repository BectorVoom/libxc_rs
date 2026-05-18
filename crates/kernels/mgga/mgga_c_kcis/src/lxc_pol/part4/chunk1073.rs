//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1073/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1073<F: Float>(t102: F, t4880: F, t4859: F, t23: F, t821: F, t6: F, t107: F, t4866: F, t2621: F, t9: F, t7: F, t118: F) -> (F, F, F, F, F) {
    let t13577 = t102 * t4880;
    let t13578 = t13577 * t4859;
    let t13581 = F::new(1.0) / t23 / t821;
    let t13582 = t6 * t13581;
    let t13583 = t107 * t13582;
    let t13584 = t13583 * t4866;
    let t13587 = F::new(1.0) / t9 / t2621;
    let t13588 = t7 * t13587;
    let t13589 = t118 * t13588;
    (t13577, t13578, t13583, t13584, t13589)
}
