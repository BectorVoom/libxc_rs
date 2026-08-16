//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1219/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1219<F: Float>(t3331: F, t6031: F, t11282: F, t6084: F, t11292: F, t4899: F, t6138: F, t6144: F, t11588: F, t1887: F, t337: F, t5416: F) -> (F, F, F, F, F, F, F, F) {
    let t64292 = t6031 * t3331;
    let t64451 = t11282 * t6084;
    let t64537 = t11292 * t6084;
    let t64644 = t4899 * t6138;
    let t64648 = t4899 * t6144;
    let t64763 = t11588 * t6138;
    let t64779 = t11588 * t6144;
    let t64811 = t5416 * t337 * t1887;
    (t64292, t64451, t64537, t64644, t64648, t64763, t64779, t64811)
}
