//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 634/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk634<F: Float>(t1336: F, t5245: F, t557: F, t67: F, t246: F, t1351: F, t3792: F, t546: F, t68: F, t3787: F, t544: F, t1338: F) -> (F, F, F, F, F, F, F) {
    let t5246 = t1336 * t5245;
    let t5247 = t557 * t67;
    let t5248 = t5247 * t246;
    let t5250 = t3792 * t1351;
    let t5278 = t546 * t68;
    let t5333 = t68 * t3787;
    let t5334 = t544 * t5333;
    let t5343 = t68 * t1338;
    (t5246, t5247, t5248, t5250, t5278, t5334, t5343)
}
