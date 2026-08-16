//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2464/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2464<F: Float>(t10470: F, t11058: F, t381: F, t1615: F, t6739: F, t10482: F, t3120: F, t11064: F, t1057: F, t49864: F, t3040: F, t4657: F) -> (F, F, F, F, F, F) {
    let t50508 = t10470 * t11058 * t381;
    let t50509 = t1615 * t6739;
    let t50510 = t10482 * t3120;
    let t50516 = t10470 * t11064 * t381;
    let t50535 = t49864 * t1057;
    let t50540 = t4657 * t3040;
    (t50508, t50509, t50510, t50516, t50535, t50540)
}
