//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 918/1064 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk918<F: Float>(t1509: F, t7510: F, t1902: F, t5584: F, t1493: F, t254: F, t225: F, t28282: F, t10143: F, t1649: F, t112: F, t29395: F) -> (F, F, F, F, F, F) {
    let t98524 = t7510 * t1509;
    let t98541 = t1902 * t5584;
    let t98975 = t1493 * t254;
    let t99010 = t28282 * t225;
    let t100688 = t10143 * t1649;
    let t100996 = t29395 * t112;
    (t98524, t98541, t98975, t99010, t100688, t100996)
}
