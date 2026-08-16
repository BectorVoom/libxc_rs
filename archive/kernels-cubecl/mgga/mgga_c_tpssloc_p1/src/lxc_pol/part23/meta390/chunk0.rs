//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1194/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1194<F: Float>(t12328: F, t1815: F, t12248: F, t1834: F, t111: F, t6470: F, t2281: F, t5489: F, t5465: F, t2239: F, t5385: F, t19681: F, t2528: F) -> (F, F, F, F, F, F, F) {
    let t54793 = t1815 * t12328;
    let t54930 = t12248 * t1834;
    let t55388 = t6470 * t111;
    let t55531 = t2281 * t5489;
    let t55537 = t2281 * t5465;
    let t55921 = t5385 * t2239;
    let t56099 = t19681 * t2528;
    (t54793, t54930, t55388, t55531, t55537, t55921, t56099)
}
