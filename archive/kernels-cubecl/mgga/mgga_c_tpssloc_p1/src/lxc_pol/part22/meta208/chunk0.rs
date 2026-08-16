//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1205/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1205<F: Float>(t3403: F, t6105: F, t1164: F, t338: F, t5416: F) -> (F, F, F) {
    let t6106 = t6105 * t3403;
    let t6108 = F::cast_from(0.17315859105681463759e2_f64) * t1164 * t6106;
    let t6109 = t5416 * t338;
    (t6106, t6108, t6109)
}
