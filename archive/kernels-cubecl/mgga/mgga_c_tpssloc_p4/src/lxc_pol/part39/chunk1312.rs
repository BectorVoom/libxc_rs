//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1312/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1312<F: Float>(t1453: F, t2358: F, t4072: F, t649: F, t12813: F, t88: F, t1458: F, t2311: F, t89: F, t626: F, t9365: F, t45435: F, t64: F) -> (F, F, F, F, F, F, F) {
    let t86598 = t1453 * t2358;
    let t90370 = t649 * t4072;
    let t90375 = t88 * t12813;
    let t90381 = t2311 * t1458;
    let t91753 = t89 * t12813;
    let t110075 = t626 * t9365;
    let t110082 = t64 * t45435;
    (t86598, t90370, t90375, t90381, t91753, t110075, t110082)
}
