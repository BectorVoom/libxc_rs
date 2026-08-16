//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 45 (v4rho2sigma2_1) CSE chunk 745/1056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part45_v4rho2sigma2_1_chunk745<F: Float>(t23046: F, t240: F, t812: F, t2635: F, t2681: F, t6614: F, t2617: F, t6613: F, t831: F, t1878: F, t244: F, t2606: F) -> (F, F, F, F, F) {
    let t23047 = t23046 * t240;
    let t23048 = t812 * t23047;
    let t23049 = t23048 * t2635;
    let t23051 = t6614 * t2681;
    let t23053 = t2617 * t6613;
    let t23054 = t23053 * t831;
    let t23056 = t1878 * t244;
    let t23057 = t23056 * t2606;
    (t23049, t23051, t23054, t23056, t23057)
}
