//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 696/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk696<F: Float>(t1384: F, t1409: F, t452: F, t454: F, t1445: F, t1453: F, t518: F, t7: F) -> (F, F, F, F, F) {
    let t4704 = t1409 * t1384;
    let t4705 = t4704 * t452;
    let t4708 = t454 * t1409;
    let t4711 = t1445 * t1453;
    let t4715 = t7 * t518;
    (t4704, t4705, t4708, t4711, t4715)
}
