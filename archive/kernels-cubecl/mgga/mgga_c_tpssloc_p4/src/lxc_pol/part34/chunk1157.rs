//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1157/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1157<F: Float>(t225: F, t29099: F, t29071: F, t29040: F, t814: F, t2047: F, t5611: F, t26959: F, t7428: F, t27979: F, t7032: F, t1860: F, t27956: F, t7031: F) -> (F, F, F, F, F, F, F) {
    let t101509 = t29099 * t225;
    let t101593 = t29071 * t225;
    let t101694 = t814 * t29040;
    let t101708 = t2047 * t5611;
    let t102137 = t7428 * t26959;
    let t102139 = t27979 * t7032;
    let t102142 = t1860 * t7031 * t27956;
    (t101509, t101593, t101694, t101708, t102137, t102139, t102142)
}
